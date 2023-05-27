use std::{collections::HashMap, ops::Range, str::FromStr, sync::Arc, io::Read};

extern crate hyper;
extern crate hyper_native_tls;

use html5ever::tendril::fmt::Slice;
use hyper::{
    body::{self, Bytes},
    header::*,
    http::uri::Scheme,
    Body, Client, Method, Request, Response, StatusCode, Uri, Version,
};
use log::error;
use regex::Regex;
use tokio::runtime::Runtime;

use crate::{
    proxy::log::{LogRequest, ReqResLog, RequestParam},
    st_error,
    utils::{STError, config::{get_config}},
};

#[derive(Debug)]
pub struct HttpRequest {
    pub(crate) request: Request<Body>,
    pub(crate) body: Arc<Bytes>,
    pub(crate) proxy: Arc<String>
}

impl HttpRequest {
    pub fn from_bytes(bs: Bytes) -> HttpRequest {
        unimplemented!()
    }

    pub fn clone(&self) -> HttpRequest {
        let mut request = Request::new(Body::from(""));
        *request.uri_mut() = self.request.uri().clone();
        *request.headers_mut() = self.request.headers().clone();
        HttpRequest {
            request: request,
            body: self.body.clone(),
            proxy: self.proxy.clone()
        }
    }

    pub fn to_bytes(&self) -> Bytes {
        unimplemented!()
    }

    pub fn set_version(&mut self, v: &Version) {
        *self.request.version_mut() = v.clone();
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
            request: request,
            body: Arc::new(Bytes::new()),
            proxy: get_config().get_proxy().clone()
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

    pub fn update_with_params(&self, params: &Vec<RequestParam>) -> Result<(), STError> {
        let mut get_map: HashMap<String, Option<String>> = HashMap::new();
        let uri = self.request.uri();
        let original = uri.query().unwrap();
        let querys = original.split("&").collect::<Vec<&str>>();
        for query in querys {
            let kv = query.split("=").collect::<Vec<&str>>();
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
                query.push(format!("{}={}", kv.0, kv.1.unwrap().to_string()));
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

    pub async fn send_async(method: Method, request: &HttpRequest) -> Result<HttpResponse, STError> {
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
            
        let rt = Runtime::new().unwrap();
        let body = reqwest::Body::from((*request.body).clone());
        let response = {
            if method.eq(&Method::GET) {
                cli.get(request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::POST) {
                cli.post(request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::OPTIONS) {
                cli.request(reqwest::Method::OPTIONS,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::PATCH) {
                cli.request(reqwest::Method::PATCH,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::DELETE) {
                cli.request(reqwest::Method::DELETE,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::HEAD) {
                cli.request(reqwest::Method::HEAD,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::PUT) {
                cli.request(reqwest::Method::PUT,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::TRACE) {
                cli.request(reqwest::Method::TRACE,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } 
            
            else {
                cli.get(request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            }
        }; 
        let ret = match response.await {
            Ok(s) => s,
            Err(e) => {
                return Err(st_error!(e));
            }
        };
        let resp = HttpResponse::from_reqwest_response_async(request, ret);
        resp
    }

    pub fn send(method: Method, request: &HttpRequest) -> Result<HttpResponse, STError> {
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
            
        let rt = Runtime::new().unwrap();
        let body = reqwest::blocking::Body::from((*request.body).clone());
        let response = {
            if method.eq(&Method::GET) {
                cli.get(request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::POST) {
                cli.post(request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::OPTIONS) {
                cli.request(reqwest::Method::OPTIONS,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::PATCH) {
                cli.request(reqwest::Method::PATCH,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::DELETE) {
                cli.request(reqwest::Method::DELETE,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::HEAD) {
                cli.request(reqwest::Method::HEAD,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::PUT) {
                cli.request(reqwest::Method::PUT,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } else if method.eq(&Method::TRACE) {
                cli.request(reqwest::Method::TRACE,request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            } 
            
            else {
                cli.get(request.request.uri().to_string()).headers(request.request.headers().clone()).body(body).send()
            }
        }; 
        let ret = response.unwrap();
        let resp = HttpResponse::from_reqwest_response(request, ret);
        resp
    }

    
}

#[derive(Debug)]
pub struct HttpResponse {
    req: HttpRequest,
    resp: Response<Body>,
    body: Bytes,
}

impl HttpResponse {
    pub fn from_reqwest_response_async(req: &HttpRequest, resp: reqwest::Response) -> Result<Self,STError> {
        let mut _resp = Response::new(Body::from(""));
        _resp.headers_mut().clone_from(resp.headers());
        let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
        let body = rt.block_on(resp.bytes()).unwrap();
        Ok(Self { req: req.clone(), 
            resp: _resp, 
            body: body
        })
    }

    pub fn from_reqwest_response(req: &HttpRequest, resp: reqwest::blocking::Response) -> Result<Self,STError> {
        let mut _resp = Response::new(Body::from(""));
        _resp.headers_mut().clone_from(resp.headers());

        let body = resp.bytes().unwrap();
        Ok(Self { req: req.clone(), 
            resp: _resp, 
            body: body
        })
    }

    pub fn from(req: &HttpRequest, resp: Response<Body>, body: Bytes) -> Self {
        Self {
            req: req.clone(),
            resp: resp,
            body: body,
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
            None => {
                return "".to_string();
            }
        }
    }

    pub fn get_headers(&self) -> &HeaderMap {
        self.resp.headers()
    }

    pub fn get_body(&self) -> &Bytes {
        &self.body
    }

    pub fn get_httplog(&self) -> ReqResLog {
        unimplemented!()
    }

    pub fn get_request(&self) -> &HttpRequest {
        &self.req
    }

    pub fn clone_original(&self) -> Response<Body> {
        let mut response = Response::new(Body::from(""));
        *response.headers_mut() = self.resp.headers().clone();
        *response.status_mut() = self.resp.status().clone();
        response
    }
}

impl HttpRequest {
    pub fn from_burp(burp: &BurpRequest) -> Result<Self, STError> {
        let domain_with_scheme: String;
        if burp.ssl {
            domain_with_scheme = format!("https://{}/", burp.host);
        } else {
            domain_with_scheme = format!("http://{}/", burp.host);
        }

        let headers = burp.headers.split("\n").collect::<Vec<&str>>();
        let method: String;
        let path: String;
        let proto: String;
        let first = headers[0].trim();
        let first = first.split(" ").collect::<Vec<&str>>();
        if first.len() != 3 {
            return Err(STError::new("first line does not match pattern"));
        }

        method = first[0].to_string();
        path = first[1].to_string();
        proto = first[2].to_string();
        let headers = headers[1..].to_vec();
        let mut header_map: HeaderMap = HeaderMap::new();
        for header in headers {
            let header = header.trim();
            let index = header.find(":");
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
            request: request,
            body: burp.body.clone(),
            proxy: get_config().get_proxy().clone()
        })
    }

    pub fn to_burp(&self) -> BurpRequest {
        let mut result: String = String::new();
        let ssl: bool;
        let _ssl = self.request.uri().scheme().unwrap();
        if _ssl.eq(&Scheme::HTTP) {
            ssl = false;
        } else {
            ssl = true;
        }

        let path = self.request.uri().path_and_query().unwrap().to_string();
        let version = self.request.version();
        let v: &str;
        if version.eq(&Version::HTTP_09) {
            v = "HTTP/0.9"
        } else if version.eq(&Version::HTTP_10) {
            v = "HTTP/0.9"
        } else if version.eq(&Version::HTTP_11) {
            v = "HTTP/0.9"
        } else if version.eq(&Version::HTTP_2) {
            v = "HTTP/0.9"
        } else if version.eq(&Version::HTTP_3) {
            v = "HTTP/0.9"
        } else {
            v = "HTTP/1.1"
        }
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
            ssl: ssl,
            host: host,
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
    pub fn clone(&self) -> BurpRequest {
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
    Body
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
    place   : BPlace
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
        let mut c = (*self).clone();
        if param.get_place().eq(&BPlace::Headers) {
            let name = c.headers[param.name_start..(param.name_end+1)].to_string();
            let value = c.headers[param.value_start..(param.value_end+1)].to_string();
            if name.eq(param.get_name()) && value.eq(param.get_value()) {
                return Ok(c);
            }

            if (!name.eq(param.get_name())) && (!value.eq(param.get_value())) {
                return Err(STError::new("Can not change two element at same time"));
            }

            if !name.eq(param.get_name()) {
                c.headers.replace_range(param.name_start..(param.name_end+1), param.get_name());
            }

            if !value.eq(param.get_value()) {
                c.headers.replace_range(param.value_start..(param.value_end+1), &param.get_value());
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
        let q_mark = first.find("?");
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
            let _s = match _tmp.find(" ") {
                Some(ss) => {
                    query_end = ss + query_base;
                    ss
                }
                None => {
                    return Err(STError::new("Format error"));
                }
            };

            query = (&first[query_base..query_end]).to_string();
        }

        let query_param = BurpParam {
            name_start: 0,
            name: "".to_string(),
            name_end: 0,
            value_start: query_base,
            value_end: query_end,
            value: query.clone(),
            param_type: BParamType::GetQuery,
            place: BPlace::Headers
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
                        place: BPlace::Headers
                    };
                    result.push(header_param);
                }
            }
        }
        let headers = &headers[1..].join("\r\n");
        let header_pattern = Regex::new(r"([\w\-]+): ?([\r\n]+)").unwrap();
        for cap in header_pattern.captures_iter(&headers) {
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
                        place: BPlace::Headers
                    };
                    result.push(header_param);
                }
            }
        }

        let kv_pattern = Regex::new(r"(\w+)=([^;\n\r]+)").unwrap();
        for cap in kv_pattern.captures_iter(&headers) {
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
                        place: BPlace::Headers
                    };
                    result.push(header_param);
                }
            }
        }
        Ok(result)
    }
}
