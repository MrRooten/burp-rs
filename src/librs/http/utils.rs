use std::{
    collections::HashMap,
    ops::Range,
    ptr::addr_of_mut,
    str::FromStr,
    sync::{Arc, Mutex},
};

extern crate hyper;

use hyper::{
    body::Bytes, header::*, http::uri::Scheme, Body, Method, Request, Response, StatusCode, Uri,
    Version,
};
use log::error;
use once_cell::sync::Lazy;
use regex::Regex;
use tokio::{runtime::Runtime, sync::Semaphore};

use crate::{
    proxy::log::{LogRequest, ReqResLog, RequestParam},
    st_error,
    utils::{config::get_config, STError},
};

#[derive(Debug)]
pub struct HttpRequest {
    pub(crate) request: Request<Body>,
    pub(crate) body: Arc<Bytes>,
    pub(crate) proxy: Arc<String>,
}

pub struct RequestPools {
    pools: HashMap<String, Arc<Semaphore>>,
    default: u32,
}

static mut REQUEST_POOLS: Lazy<RequestPools> = Lazy::new(RequestPools::new);

static mut REQ_RT: Lazy<Runtime> = Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

static mut _REQUEST_POOLS_LOCK: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

fn get_pools_lock() -> &'static mut Lazy<Mutex<u32>> {
    unsafe { &mut *addr_of_mut!(_REQUEST_POOLS_LOCK) }
}

pub fn get_req_rt() -> &'static mut Lazy<Runtime> {
    unsafe { &mut *addr_of_mut!(REQ_RT) }
}

impl Default for RequestPools {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestPools {
    pub fn new() -> Self {
        let num = get_config()
            .get("http.parallel_per_site")
            .as_i64()
            .unwrap_or(5);

        Self {
            pools: HashMap::new(),
            default: num as u32,
        }
    }

    pub fn get_sem(&mut self, host: &str) -> &Semaphore {
        let lock = get_pools_lock();
        if self.pools.contains_key(host) {
            return self.pools.get(host).unwrap();
        }

        self.pools.insert(
            host.to_string(),
            Arc::new(Semaphore::new(self.default as usize)),
        );
        return self.pools.get(host).unwrap();
    }

    pub fn get_pools() -> &'static mut Lazy<RequestPools> {
        unsafe { &mut *addr_of_mut!(REQUEST_POOLS) }
    }

    pub fn resize_pool(&mut self, host: &str, num: u32) {
        self.pools.insert(
            host.to_string(),
            Arc::new(Semaphore::new(self.default as usize)),
        );
    }
}

impl HttpRequest {
    pub fn from_bytes(bs: Bytes) -> HttpRequest {
        unimplemented!()
    }

    pub fn clone_from_request(&self) -> HttpRequest {
        let mut request = Request::new(Body::from(""));
        *request.uri_mut() = self.request.uri().clone();
        *request.headers_mut() = self.request.headers().clone();
        HttpRequest {
            request,
            body: self.body.clone(),
            proxy: self.proxy.clone(),
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        unimplemented!()
    }

    pub fn set_version(&mut self, v: &Version) {
        *self.request.version_mut() = *v;
    }

    pub fn from_url(url: &str) -> Result<HttpRequest, STError> {
        let mut request = Request::new(Body::from(""));
        *request.uri_mut() = match Uri::from_str(url) {
            Ok(o) => o,
            Err(e) => {
                error!("invalid url: {}", url);
                return Err(st_error!(e));
            }
        };

        Ok(HttpRequest {
            request,
            body: Arc::new(Bytes::new()),
            proxy: get_config().get_proxy().clone(),
        })
    }

    pub fn clone_origial(&self) -> Request<Body> {
        let mut request = Request::new(Body::from(""));
        *request.uri_mut() = self.request.uri().clone();
        *request.headers_mut() = self.request.headers().clone();
        request
    }

    pub fn get_body(&self) -> &Arc<Bytes> {
        &self.body
    }

    pub fn from_log_request(request: &LogRequest) -> HttpRequest {
        request.to_http_request()
    }

    pub fn update_with_params(&self, params: &[RequestParam]) -> Result<(), STError> {
        let mut get_map: HashMap<String, Option<String>> = HashMap::new();
        let uri = self.request.uri();
        let original = uri.query().unwrap();
        let querys = original.split('&').collect::<Vec<&str>>();
        for query in querys {
            let kv = query.split('=').collect::<Vec<&str>>();
            if kv.len() == 2 {
                get_map.insert(kv[0].to_string(), Some(kv[1].to_string()));
            }

            if kv.len() == 1 {
                get_map.insert(kv[0].to_string(), None);
            }
        }

        let mut query = Vec::<String>::default();
        for kv in get_map {
            if kv.1.is_none() {
                query.push(kv.0.to_string());
            }

            if kv.1.is_some() {
                query.push(format!("{}={}", kv.0, kv.1.unwrap()));
            }
        }

        let query = query.join("&");
        unimplemented!()
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.request.headers_mut().append(
            HeaderName::from_str(key).unwrap(),
            HeaderValue::from_str(value).unwrap(),
        );
    }

    pub fn remove_header(&mut self, key: &str) {
        self.request
            .headers_mut()
            .remove(HeaderName::from_str(key).unwrap());
    }
    pub fn set_headers(&mut self, headers: &HeaderMap) {
        *self.request.headers_mut() = headers.clone();
    }
    pub fn set_body(&mut self, body: Arc<Bytes>) {
        self.body = body.clone();
    }

    fn set_method(&mut self, method: Method) {
        *self.request.method_mut() = method;
    }

    pub async fn send2(method: Method, request: HttpRequest) -> Result<HttpResponse, STError> {
        let h = tokio::spawn(async move {
            match HttpRequest::send_async(method, request).await {
                Ok(s) => Ok(s),
                Err(e) => Err(e),
            }
        });
        let result = h.await;

        match result {
            Ok(s) => s,
            Err(e) => Err(st_error!(e)),
        }
    }

    pub async fn send_async(method: Method, request: HttpRequest) -> Result<HttpResponse, STError> {
        let host = match request.request.uri().host() {
            Some(s) => s,
            None => return Err(STError::new("No host in request")),
        };

        let cli = {
            if request.proxy.len() == 0 {
                reqwest::Client::new()
            } else {
                let proxy = match reqwest::Proxy::http(request.proxy.as_str()) {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(st_error!(e));
                    }
                };
                match reqwest::Client::builder().proxy(proxy).build() {
                    Ok(o) => o,
                    Err(e) => {
                        return Err(st_error!(e));
                    }
                }
            }
        };
        let pools = RequestPools::get_pools();
        let sem = pools.get_sem(host);
        let _ = sem.acquire().await;

        let body = reqwest::Body::from((*request.body).clone());
        let response = {
            if method.eq(&Method::GET) {
                cli.get(request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::POST) {
                cli.post(request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::OPTIONS) {
                cli.request(reqwest::Method::OPTIONS, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::PATCH) {
                cli.request(reqwest::Method::PATCH, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::DELETE) {
                cli.request(reqwest::Method::DELETE, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::HEAD) {
                cli.request(reqwest::Method::HEAD, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::PUT) {
                cli.request(reqwest::Method::PUT, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::TRACE) {
                cli.request(reqwest::Method::TRACE, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else {
                cli.get(request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            }
        };
        let ret = match response.await {
            Ok(s) => s,
            Err(e) => {
                return Err(st_error!(e));
            }
        };
        let resp = HttpResponse::from_reqwest_response_async(request, ret);
        resp.await
    }

    pub fn send(method: Method, request: HttpRequest) -> Result<HttpResponse, STError> {
        let host = match request.request.uri().host() {
            Some(s) => s,
            None => return Err(STError::new("No host in request")),
        };

        let cli = {
            if request.proxy.len() == 0 {
                reqwest::blocking::Client::new()
            } else {
                let proxy = match reqwest::Proxy::http(request.proxy.as_str()) {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(st_error!(e));
                    }
                };
                match reqwest::blocking::Client::builder().proxy(proxy).build() {
                    Ok(o) => o,
                    Err(e) => {
                        return Err(st_error!(e));
                    }
                }
            }
        };
        let pools = RequestPools::get_pools();
        let sem = pools.get_sem(host);
        let _d = sem.acquire();
        let body = reqwest::blocking::Body::from((*request.body).clone());
        let response = {
            if method.eq(&Method::GET) {
                cli.get(request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::POST) {
                cli.post(request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::OPTIONS) {
                cli.request(reqwest::Method::OPTIONS, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::PATCH) {
                cli.request(reqwest::Method::PATCH, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::DELETE) {
                cli.request(reqwest::Method::DELETE, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::HEAD) {
                cli.request(reqwest::Method::HEAD, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::PUT) {
                cli.request(reqwest::Method::PUT, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else if method.eq(&Method::TRACE) {
                cli.request(reqwest::Method::TRACE, request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            } else {
                cli.get(request.request.uri().to_string())
                    .headers(request.request.headers().clone())
                    .body(body)
                    .send()
            }
        };
        let ret = match response {
            Ok(s) => s,
            Err(e) => {
                return Err(st_error!(e));
            }
        };

        HttpResponse::from_reqwest_response(&request, ret)
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    req: HttpRequest,
    resp: Response<Body>,
    body: Bytes,
}

impl HttpResponse {
    pub async fn from_reqwest_response_async(
        req: HttpRequest,
        resp: reqwest::Response,
    ) -> Result<Self, STError> {
        let mut _resp = Response::new(Body::from(""));
        _resp.headers_mut().clone_from(resp.headers());
        let body = match resp.bytes().await {
            Ok(s) => s,
            Err(e) => {
                return Err(st_error!(e));
            }
        };
        Ok(Self {
            req,
            resp: _resp,
            body,
        })
    }

    pub fn from_reqwest_response(
        req: &HttpRequest,
        resp: reqwest::blocking::Response,
    ) -> Result<Self, STError> {
        let mut _resp = Response::new(Body::from(""));
        _resp.headers_mut().clone_from(resp.headers());

        let body = resp.bytes().unwrap();
        Ok(Self {
            req: req.clone_from_request(),
            resp: _resp,
            body,
        })
    }

    pub fn from(req: HttpRequest, resp: Response<Body>, body: Bytes) -> Self {
        Self {
            req: req.clone_from_request(),
            resp,
            body,
        }
    }

    pub fn get_status(&self) -> StatusCode {
        self.resp.status()
    }

    pub fn get_header(&self, key: &str) -> String {
        let c_type = self.resp.headers().get(key);
        match c_type {
            Some(s) => {
                return s.to_str().unwrap().to_string();
            }
            None => "".to_string(),
        }
    }

    pub fn get_headers(&self) -> &HeaderMap {
        self.resp.headers()
    }

    pub fn get_body(&self) -> &Bytes {
        &self.body
    }

    pub fn get_httplog(&self) -> ReqResLog {
        ReqResLog::from_http_response(self)
    }

    pub fn get_request(&self) -> &HttpRequest {
        &self.req
    }

    pub fn clone_original(&self) -> Response<Body> {
        let mut response = Response::new(Body::from(""));
        *response.headers_mut() = self.resp.headers().clone();
        *response.status_mut() = self.resp.status();
        response
    }
}

impl HttpRequest {
    pub fn from_burp(burp: &BurpRequest) -> Result<Self, STError> {
        let domain_with_scheme = if burp.ssl {
            format!("https://{}/", burp.host)
        } else {
            format!("http://{}/", burp.host)
        };

        let headers = burp.headers.split('\n').collect::<Vec<&str>>();

        let first = headers[0].trim();
        let first = first.split(' ').collect::<Vec<&str>>();
        if first.len() != 3 {
            return Err(STError::new("first line does not match pattern"));
        }

        let method: String = first[0].to_string();
        let path: String = first[1].to_string();
        let proto: String = first[2].to_string();
        let headers = headers[1..].to_vec();
        let mut header_map: HeaderMap = HeaderMap::new();
        for header in headers {
            let header = header.trim();
            let index = header.find(':');
            let index = match index {
                Some(s) => s,
                None => {
                    let msg = format!("header '{}' does not match", header);
                    return Err(STError::new(&msg));
                }
            };

            let key = &header[0..index];
            let value = &header[index + 1..];
            header_map.append(
                HeaderName::from_str(key).unwrap(),
                HeaderValue::from_str(value).unwrap(),
            );
        }
        let mut request = Request::new(Body::from(""));
        *request.headers_mut() = header_map;
        let m: Method;
        if method.eq_ignore_ascii_case("GET") {
            m = Method::GET;
        } else if method.eq_ignore_ascii_case("POST") {
            m = Method::POST;
        } else if method.eq_ignore_ascii_case("OPTIONS") {
            m = Method::OPTIONS;
        } else if method.eq_ignore_ascii_case("PUT") {
            m = Method::PUT;
        } else if method.eq_ignore_ascii_case("DELETE") {
            m = Method::DELETE;
        } else if method.eq_ignore_ascii_case("HEAD") {
            m = Method::HEAD;
        } else if method.eq_ignore_ascii_case("PATCH") {
            m = Method::PATCH;
        } else if method.eq_ignore_ascii_case("TRACE") {
            m = Method::TRACE;
        } else {
            m = Method::DELETE;
        }
        let uri = Uri::from_str(&format!("{}{}", domain_with_scheme, path)).unwrap();
        *request.method_mut() = m;
        *request.uri_mut() = uri;
        let p: Version;
        if proto.eq("HTTP/0.9") {
            p = Version::HTTP_09;
        } else if proto.eq("HTTP/1.0") {
            p = Version::HTTP_10;
        } else if proto.eq("HTTP/1.1") {
            p = Version::HTTP_11;
        } else if proto.eq("HTTP/2") {
            p = Version::HTTP_2;
        } else if proto.eq("HTTP/3") {
            p = Version::HTTP_3;
        } else {
            p = Version::HTTP_11;
        }

        *request.version_mut() = p;

        Ok(Self {
            request,
            body: burp.body.clone(),
            proxy: get_config().get_proxy().clone(),
        })
    }

    pub fn to_burp(&self) -> BurpRequest {
        let mut result: String = String::new();

        let _ssl = self.request.uri().scheme().unwrap();
        let ssl: bool = !_ssl.eq(&Scheme::HTTP);

        let path = self.request.uri().path_and_query().unwrap().to_string();
        let version = self.request.version();
        let v = if version.eq(&Version::HTTP_3)
            || version.eq(&Version::HTTP_09)
            || version.eq(&Version::HTTP_10)
            || version.eq(&Version::HTTP_11)
            || version.eq(&Version::HTTP_2)
        {
            "HTTP/0.9"
        } else {
            "HTTP/1.1"
        };
        let m: &str;
        let method = self.request.method();
        if method.eq(&Method::GET) {
            m = "GET";
        } else if method.eq(&Method::POST) {
            m = "POST";
        } else if method.eq(&Method::PATCH) {
            m = "PATCH";
        } else if method.eq(&Method::DELETE) {
            m = "DELETE";
        } else if method.eq(&Method::PUT) {
            m = "PUT";
        } else if method.eq(&Method::TRACE) {
            m = "TRACE";
        } else if method.eq(&Method::OPTIONS) {
            m = "OPTIONS";
        } else if method.eq(&Method::HEAD) {
            m = "HEAD";
        } else {
            m = "CONNECT";
        }

        result.push_str(&format!("{} {} {}\r\n", m, path, v));
        let _host = self.request.uri().host().unwrap();
        let host: String;
        match self.request.uri().port_u16() {
            Some(s) => {
                host = format!("{}:{}", _host, s);
            }
            None => {
                if ssl {
                    host = format!("{}:443", _host);
                } else {
                    host = format!("{}:80", _host);
                }
            }
        }

        let headers = self.request.headers();
        for key in headers.keys() {
            let values = headers
                .get_all(key)
                .iter()
                .map(|v| v.to_str().unwrap())
                .collect::<Vec<&str>>()
                .join(";");
            result.push_str(&format!("{}: {}\r\n", key.as_str(), values));
        }
        BurpRequest {
            headers: result,
            body: self.body.clone(),
            ssl,
            host,
        }
    }
}

#[derive(Debug)]
pub struct BurpRequest {
    headers: String,
    body: Arc<Bytes>,
    ssl: bool,
    host: String,
}

impl BurpRequest {
    pub fn clone_from_burp_request(&self) -> BurpRequest {
        Self {
            headers: self.headers.clone(),
            body: Arc::new((*self.body).clone()),
            ssl: self.ssl,
            host: self.host.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BParamType {
    Get,
    GetQuery,
    Header,
    HeaderValue,
    Cookie,
    Json,
    Xml,
    Post,
}

#[derive(Debug, PartialEq)]
pub enum BPlace {
    Headers,
    Body,
}

#[derive(Debug)]
pub struct BurpParam {
    name_start: usize,
    name: String,
    name_end: usize,
    value_start: usize,
    value_end: usize,
    value: String,
    param_type: BParamType,
    place: BPlace,
}

impl BurpParam {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }

    pub fn get_name_range(&self) -> Range<usize> {
        Range {
            start: self.name_start,
            end: self.name_end,
        }
    }

    pub fn get_value_range(&self) -> Range<usize> {
        Range {
            start: self.value_start,
            end: self.value_end,
        }
    }

    pub fn get_type(&self) -> &BParamType {
        &self.param_type
    }

    pub fn get_place(&self) -> &BPlace {
        &self.place
    }
}

impl BurpRequest {
    pub fn replace(&mut self, start: usize, end: usize, s: &str) {
        if start < self.headers.len() && end < self.headers.len() {
            self.headers.replace_range(start..end, s);
        }
    }

    pub fn replace_param(&mut self, param: &BurpParam) -> Result<BurpRequest, STError> {
        let mut c = (*self).clone_from_burp_request();
        if param.get_place().eq(&BPlace::Headers) {
            let name = c.headers[param.name_start..(param.name_end + 1)].to_string();
            let value = c.headers[param.value_start..(param.value_end + 1)].to_string();
            if name.eq(param.get_name()) && value.eq(param.get_value()) {
                return Ok(c);
            }

            if (!name.eq(param.get_name())) && (!value.eq(param.get_value())) {
                return Err(STError::new("Can not change two element at same time"));
            }

            if !name.eq(param.get_name()) {
                c.headers
                    .replace_range(param.name_start..(param.name_end + 1), param.get_name());
            }

            if !value.eq(param.get_value()) {
                c.headers
                    .replace_range(param.value_start..(param.value_end + 1), param.get_value());
            }
        }
        Ok(c)
    }

    pub fn get_params(&self) -> Result<Vec<BurpParam>, STError> {
        let mut result = vec![];
        let headers = self.headers.split("\r\n").collect::<Vec<&str>>();
        let first = headers[0];
        let mut query_base: usize = 0;
        let mut query_end: usize = 0;
        let q_mark = first.find('?');
        let s = match q_mark {
            Some(o) => {
                query_base = o + 1;
                o
            }
            None => 0,
        };

        let query: String;

        if s == 0 {
            query = "".to_string();
        } else {
            let _tmp = &first[(query_base)..first.len()];
            let _s = match _tmp.find(' ') {
                Some(ss) => {
                    query_end = ss + query_base;
                    ss
                }
                None => {
                    return Err(STError::new("Format error"));
                }
            };

            query = first[query_base..query_end].to_string();
        }

        let query_param = BurpParam {
            name_start: 0,
            name: "".to_string(),
            name_end: 0,
            value_start: query_base,
            value_end: query_end,
            value: query.clone(),
            param_type: BParamType::GetQuery,
            place: BPlace::Headers,
        };
        result.push(query_param);
        let get_pattern = Regex::new(r"(\w+)=([^;\n\r]+)").unwrap();
        for cap in get_pattern.captures_iter(first) {
            let key = cap.get(1);
            let value = cap.get(2);
            if let Some(k) = key {
                if let Some(v) = value {
                    let header_param = BurpParam {
                        name_start: k.start() + first.len() + 2,
                        name: k.as_str().to_string(),
                        name_end: k.end() + first.len() + 2,
                        value_start: v.start() + first.len() + 2,
                        value_end: v.end() + first.len() + 2,
                        value: v.as_str().to_string(),
                        param_type: BParamType::Get,
                        place: BPlace::Headers,
                    };
                    result.push(header_param);
                }
            }
        }
        let headers = &headers[1..].join("\r\n");
        let header_pattern = Regex::new(r"([\w\-]+): ?([\r\n]+)").unwrap();
        for cap in header_pattern.captures_iter(headers) {
            let key = cap.get(1);
            let value = cap.get(2);
            if let Some(k) = key {
                if let Some(v) = value {
                    let header_param = BurpParam {
                        name_start: k.start() + first.len() + 2,
                        name: k.as_str().to_string(),
                        name_end: k.end() + first.len() + 2,
                        value_start: v.start() + first.len() + 2,
                        value_end: v.end() + first.len() + 2,
                        value: v.as_str().to_string(),
                        param_type: BParamType::Header,
                        place: BPlace::Headers,
                    };
                    result.push(header_param);
                }
            }
        }

        let kv_pattern = Regex::new(r"(\w+)=([^;\n\r]+)").unwrap();
        for cap in kv_pattern.captures_iter(headers) {
            let key = cap.get(1);
            let value = cap.get(2);
            if let Some(k) = key {
                if let Some(v) = value {
                    let header_param = BurpParam {
                        name_start: k.start() + first.len() + 2,
                        name: k.as_str().to_string(),
                        name_end: k.end() + first.len() + 2,
                        value_start: v.start() + first.len() + 2,
                        value_end: v.end() + first.len() + 2,
                        value: v.as_str().to_string(),
                        param_type: BParamType::HeaderValue,
                        place: BPlace::Headers,
                    };
                    result.push(header_param);
                }
            }
        }
        Ok(result)
    }
}
