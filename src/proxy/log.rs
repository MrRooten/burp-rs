#![allow(dead_code)]
use crate::librs::http::utils::HttpResponse;
use crate::librs::object::object::IObject;
use crate::modules::Issue;
use crate::utils::utils::tidy_html;
use crate::utils::STError;
use chrono::{DateTime, Utc};
use colored::Colorize;
use flate2::read::GzDecoder;
use http::Request;
use hudsucker::hyper::{Body, Response};
use hyper::body::Bytes;
use hyper::{http, StatusCode, Version, Method, Uri};
use log::error;
use serde_json::{Value, Error};
use std::collections::{HashMap};
use std::io::Read;
use std::str::FromStr;
use std::sync::{Mutex, Arc};
use url::Url;

#[derive(Debug)]
pub struct ReqResLog {
    request: Option<LogRequest>,
    response: Option<LogResponse>,
    record_t: DateTime<Utc>,
}

impl ReqResLog {
    pub fn from_http_response(response: &HttpResponse) -> ReqResLog {
        let request = response.get_request();
        let request = LogRequest {
            orignal: request.clone_origial(),
            body: request.get_body().clone(),
            record_t: Utc::now(),
        };
        
        let resp = LogResponse {
            orignal: response.clone_original(),
            body: response.get_body().clone(),
            c_type: response.get_header("content-type"),
        };
        ReqResLog {
            request: Some(request),
            response: Some(resp),
            record_t: Utc::now(),
        }
    }

    pub fn new(req: LogRequest) -> Self {
        ReqResLog {
            request: Some(req),
            response: None,
            record_t: Utc::now(),
        }
    }

    pub fn get_host(&self) -> String {
        let request = match &self.request {
            Some(r) => {
                r
            }
            None => {
                return "".to_string();
            }
        };

        request.get_host()
    }

    pub fn set_resp(&self, resp: LogResponse) {
        i_to_m(self).response = Some(resp);
    }

    pub fn get_request(&self) -> Option<&LogRequest> {
        match &self.request {
            Some(r) => {
                return Some(r);
            }
            None => {
                return None;
            }
        }
    }

    pub fn get_size(&self) -> usize {
        let request = match &self.request {
            Some(r) => r,
            None => {
                return 0;
            }
        };

        let response = match &self.response {
            Some(r) => r,
            None => {
                return request.body.len();
            }
        };

        let ret = request.body.len() + response.body.len();

        return ret;
    }

    pub fn get_response(&self) -> Option<&LogResponse> {
        match &self.response {
            Some(r) => {
                return Some(r);
            }
            None => {
                return None;
            }
        }
    }

    pub fn clone(&self) -> Option<ReqResLog> {
        let s = match &self.request {
            Some(s) => s,
            None => {
                return None;
            }
        };

        let log = ReqResLog::new(s.clone());
        let s = match &self.response {
            Some(s) => s,
            None => {
                return Some(log);
            }
        };

        log.set_resp(s.clone());
        return Some(log);
    }
}

#[derive(Debug, PartialEq)]
pub enum ParamType {
    Get,
    GetRaw,
    Header,
    Cookie,
    Post,
    PostRaw,
    Json,
    Xml,
    
}

#[derive(Debug)]
pub struct RequestParam {
    param_type  : ParamType,
    key         : String,
    value       : String,
    json        : Value
}

impl RequestParam {
    pub fn new(param_t: ParamType, key: &str, value: &str) -> Self {
        Self { param_type: param_t, key: key.to_string(), value: value.to_string(), json: Value::default() }
    }

    pub fn from_json(v: Value) -> Self {
        Self { param_type: ParamType::Json, key: "".to_string(), value: "".to_string(), json: v }
    }

    pub fn get_param_type(&self) -> &ParamType {
        &self.param_type
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_json(&self) -> String {
        self.json.to_string()
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}

pub struct MultiPart {

}

impl MultiPart {
    pub fn new(body: &Bytes, boundary: String) -> Self {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct LogRequest {
    orignal: Request<Body>,
    body: Bytes,
    record_t: DateTime<Utc>,
}


impl IObject for LogRequest {
    fn get_object(&self, path: &str) -> Option<String> {
        let path = path.trim();
        if path.len() == 0 {
            let s = format!("{:?}", vec!["url", "headers", "body", "method", "protocol"]);
            return Some(s);
        }
        let spl = path.split(".").collect::<Vec<&str>>();
        if spl.len() == 0 {
            let s = format!("{:?}", vec!["url", "headers", "body", "method", "protocol"]);
            return Some(s);
        }

        let s1 = spl[0];
        if s1.eq("uri") || s1.eq("url") {
            let s = self.orignal.uri().to_string();
            return Some(s);
        } else if s1.eq("headers") {
            if spl.len() == 1 {
                let s = format!("{:?}", self.orignal.headers());
                return Some(s);
            }
            let s2 = spl[1];
            let value = self.get_header(s2);
            let value = match value {
                Some(v) => v,
                None => {
                    return None;
                }
            };

            return Some(value);
        } else if s1.eq("body") {
            return Some(String::from_utf8_lossy(&self.body).to_string());
        } else if s1.eq("method") {
            return Some(self.orignal.method().to_string());
        } else if s1.eq("protocol") {
            let s = format!("{:?}", self.orignal.version());
            return Some(s);
        }
        return None;
    }
}

impl LogRequest {
    pub fn from(req: Request<Body>, body: Bytes) -> LogRequest {
        LogRequest {
            orignal: req,
            body: body,
            record_t: Utc::now(),
        }
    }


    pub fn clone(&self) -> LogRequest {
        let mut new_req = Request::new(Body::from(""));
        new_req.headers_mut().clone_from(self.orignal.headers());
        new_req.method_mut().clone_from(self.orignal.method());
        new_req.uri_mut().clone_from(self.orignal.uri());
        new_req.version_mut().clone_from(&self.orignal.version());
        new_req.extensions().clone_from(&self.orignal.extensions());
        LogRequest {
            orignal: new_req,
            body: Bytes::from(self.body.clone()),
            record_t: self.record_t.clone(),
        }
    }

    pub fn get_headers(&self) -> &hyper::HeaderMap {
        self.orignal.headers()
    }

    pub fn get_url(&self) -> String {
        self.orignal.uri().to_string()
    }

    pub fn get_proto(&self) -> Version {
        self.orignal.version()
    }

    pub fn get_method(&self) -> String {
        if self.orignal.method().eq(&Method::GET) {
            return "get".to_string();
        } else if self.orignal.method().eq(&Method::POST) {
            return "post".to_string();
        } else if self.orignal.method().eq(&Method::PATCH) {
            return "patch".to_string();
        } else if self.orignal.method().eq(&Method::PUT) {
            return "put".to_string();
        } else if self.orignal.method().eq(&Method::DELETE) {
            return "delete".to_string();
        } else if self.orignal.method().eq(&Method::OPTIONS) {
            return "options".to_string();
        }

        return "get".to_string();
    }
    fn update_params(&mut self, params: &Vec<RequestParam>) {
        
    }

    pub fn set_param(&mut self, param: RequestParam) {
        let mut params = self.get_params();
        let mut found = false;
        for p in &mut params {
            if p.param_type == param.param_type && (p.key.eq(&param.key) || p.param_type == ParamType::Json){
                found = true;
                if p.param_type == ParamType::Json {
                    p.value = param.value.to_string();
                }
            }
        }

        if found == false {
            params.push(param);
        }

        self.update_params(&params);
        unimplemented!()
    }

    pub fn get_params(&self) -> Vec<RequestParam> {
        let mut ret = Vec::new();
        let url = Url::parse(&self.get_url()).unwrap();
        let query = url.query_pairs();
        for kv in query {
            ret.push(RequestParam::new(ParamType::Get, kv.0.as_ref(), kv.1.as_ref()));
        }

        let cookies = self.get_header_array("cookie");
        for cookie in cookies {
            let sp = cookie.split("=").collect::<Vec<&str>>();
            if sp.len() != 2 {
                continue;
            }

            let key = sp[0];
            let value = sp[1];
            ret.push(RequestParam::new(ParamType::Cookie, key, value));
        }

        let con_type = self.get_header("content-type");
        let con_type = match con_type {
            Some(s) => s,
            None => {
                return ret;
            }
        };

        if con_type.to_lowercase().contains("application/xml") {

        } else if con_type.to_lowercase().contains("application/json") {
            let body = String::from_utf8_lossy(&self.body).to_string();
            let json: Result<Value,Error> = serde_json::from_str(&body);
            let json = match json {
                Ok(o) => o,
                Err(e) => {
                    return ret;
                }
            };
            ret.push(RequestParam::from_json(json));
            
        } else if con_type.to_lowercase().contains("multipart/form-data") {
            let s = self.get_header("content-type").unwrap();
            let ss = s.split(";").collect::<Vec<&str>>();
            let mut boundary = String::new();
            for t in ss {
                let t = t.trim();
                if t.starts_with("boundary") {
                    let kv = t.split("=").collect::<Vec<&str>>();
                    if kv.len() == 2 {
                        boundary = kv[1].to_string();
                    }
                }
            }

            if boundary.len() == 0 {
                return ret;
            }

            let multipart = MultiPart::new(&self.body, boundary);
        } else if con_type.to_lowercase().contains("application/x-www-form-urlencoded") {
            let body = String::from_utf8_lossy(&self.body).to_string();
            let params = body.split("&").collect::<Vec<&str>>();
            for param in params {
                let kv = param.split("=").collect::<Vec<&str>>();
                if kv.len() == 2 {
                    ret.push(RequestParam::new(ParamType::Post, kv[0], kv[1]));
                } else if kv.len() == 1 {
                    ret.push(RequestParam::new(ParamType::Post, "", kv[0]));
                }
            }
        } else if con_type.to_lowercase().contains("text/plain") {
            
        }

        ret
    }
    
    /** `get_http_domain` Get log domain with http: http://baidu.com


    ```
    log.get_http_domain();
    ```
    */
    pub fn get_http_domain(&self) -> String {
        let s_url = self.get_url();
        let url = Url::parse(&s_url).unwrap();
        let schema = url.scheme();
        let mut url_s = url.host_str().unwrap().to_string();
        let mut result = String::new();
        if url.port().is_some() {
            let port = format!(":{}", url.port().unwrap());
            url_s.push_str(&port);
        }
        if schema.eq("http") {
            result.push_str("http://");
            result.push_str(&url_s);
        } else if schema.eq("https") {
            result.push_str("https://");
            result.push_str(&url_s);
        }
        return result;
    }
    /** `get_host` Return url's host ,append port to the end


    ```
    let host = request.get_host()
    assert_eq!("google.com:443",host)
    ```
    */
    pub fn get_host(&self) -> String {
        let s_url = self.get_url();
        let url = Url::parse(&s_url).unwrap();
        let schema = url.scheme();
        let mut url_s = url.host_str().unwrap().to_string();
        if url.port().is_some() {
            let port = format!(":{}", url.port().unwrap());
            url_s.push_str(&port);
        }

        url_s
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        let mut ret = String::new();
        let values = self.orignal.headers().get_all(key);
        for value in values {
            match value.to_str() {
                Ok(v) => {
                    ret.push_str(v);
                    ret.push_str(";");
                }
                Err(e) => {
                    return None;
                }
            }
        }

        Some(ret)
    }

    pub fn get_header_array(&self, key: &str) -> Vec<String> {
        let mut ret = Vec::default();
        let values = self.orignal.headers().get_all(key);
        for value in values {
            match value.to_str() {
                Ok(v) => {
                    ret.push(v.to_string());
                }
                Err(e) => {
                    continue;
                }
            }
        }

        ret
    }
    pub fn get_cookie(&self, key: &str) -> Option<String> {
        return self.get_header(key);
    }

    pub fn get_body(&self) -> &Bytes {
        return &self.body;
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        ret.push_str(&self.orignal.method().to_string());
        ret.push_str(" ");
        ret.push_str(self.orignal.uri().path_and_query().unwrap().as_str());
        ret.push_str(" ");
        if self.orignal.version() == Version::HTTP_09 {
            ret.push_str("HTTP/0.9");
        } else if self.orignal.version() == Version::HTTP_10 {
            ret.push_str("HTTP/0.9");
        } else if self.orignal.version() == Version::HTTP_2 {
            ret.push_str("HTTP/2");
        } else if self.orignal.version() == Version::HTTP_3 {
            ret.push_str("HTTP/3");
        }

        ret.push_str("\r\n");

        for kv in self.orignal.headers() {
            ret.push_str(kv.0.as_str());
            ret.push_str(": ");
            ret.push_str(kv.1.to_str().unwrap());
            ret.push_str("\r\n");
        }
        ret.push_str("\r\n");
        return ret;
    }
}

#[derive(Debug)]
pub struct LogResponse {
    orignal: Response<Body>,
    body: Bytes,
    c_type: String,
}

impl LogResponse {
    pub fn from(res: Response<Body>, body: Bytes) -> LogResponse {
        let content_type = res.headers().get("content-type");
        let content_type = match content_type {
            Some(c) => c.to_str().unwrap().to_string(),
            None => {
                return LogResponse {
                    orignal: res,
                    body: body,
                    c_type: "".to_string(),
                };
            }
        };
        LogResponse {
            orignal: res,
            body: body,
            c_type: content_type,
        }
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        let mut ret = String::new();
        let values = self.orignal.headers().get_all(key);
        for value in values {
            match value.to_str() {
                Ok(v) => {
                    ret.push_str(v);
                    ret.push_str(";");
                }
                Err(e) => {
                    return None;
                }
            }
        }

        Some(ret)
    }

    pub fn get_headers(&self) -> &hyper::HeaderMap {
        self.orignal.headers()
    }

    pub fn get_size(&self) -> usize {
        return self.body.len();
    }
    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        ret.push_str(&self.orignal.status().to_string());
        ret.push_str("\r\n");
        for kv in self.orignal.headers() {
            ret.push_str(kv.0.as_str());
            ret.push_str(": ");
            ret.push_str(kv.1.to_str().unwrap());
            ret.push_str("\r\n");
        }
        ret.push_str("\r\n");
        ret.push_str(&self.get_body_string());
        ret
    }

    pub fn get_beauty_string(&self) -> String {
        let mut ret = String::new();
        ret.push_str(&self.orignal.status().to_string().green());
        ret.push_str("\n");
        for kv in self.orignal.headers() {
            let key = kv.0.as_str();
            ret.push_str(&key.blue());
            ret.push_str(": ");
            ret.push_str(&kv.1.to_str().unwrap().red());
            ret.push_str("\n");
        }
        ret.push_str("\n");
        let c_type = self.get_header("content-type");
        let body = match c_type {
            Some(c) => {
                if c.contains("html") {
                    let s = tidy_html(&self.get_body_string());
                    s
                } else if c.contains("json") {
                    let (s, _) = prettify_js::prettyprint(&self.get_body_string());
                    s
                } else if c.contains("javascript") {
                    let (s, _) = prettify_js::prettyprint(&self.get_body_string());
                    s
                } else {
                    self.get_body_string()
                }
            }
            None => self.get_body_string(),
        };
        ret.push_str(&body);
        ret
    }

    pub fn get_status(&self) -> StatusCode {
        self.orignal.status()
    }

    pub fn get_body(&self) -> &Bytes {
        &self.body
    }

    pub fn clone(&self) -> Self {
        let mut new_res = Response::new(Body::from(""));
        new_res.extensions().clone_from(&self.orignal.extensions());
        new_res.headers_mut().clone_from(self.orignal.headers());
        new_res.status_mut().clone_from(&self.orignal.status());
        new_res.version_mut().clone_from(&self.orignal.version());
        return Self {
            orignal: new_res,
            body: self.body.clone(),
            c_type: self.c_type.clone(),
        };
    }
    fn find_subsequence(&self, haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack
            .windows(needle.len())
            .position(|window| window == needle)
    }
    pub fn contains(&self, s: &str, ignore_case: bool) -> bool {
        if ignore_case {
            if self
                .orignal
                .status()
                .to_string()
                .to_lowercase()
                .contains(&s.to_lowercase())
            {
                return true;
            }

            for kv in self.orignal.headers() {
                if kv.0.to_string().to_lowercase().contains(&s.to_lowercase()) {
                    return true;
                }

                match kv.1.to_str() {
                    Ok(o) => {
                        if o.to_lowercase().contains(&o.to_lowercase()) {
                            return true;
                        }
                    }
                    Err(e) => {}
                }
            }

            let find = self.find_subsequence(&self.body, s.as_bytes());
            if find.is_some() {
                return true;
            }

            let body_s = String::from_utf8_lossy(&self.body).to_string();
            if body_s.to_lowercase().contains(&s.to_lowercase()) {
                return true;
            }

            return false;
        } else {
            if self.orignal.status().to_string().contains(s) {
                return true;
            }

            for kv in self.orignal.headers() {
                if kv.0.to_string().contains(s) {
                    return true;
                }

                match kv.1.to_str() {
                    Ok(o) => {
                        if o.contains(s) {
                            return true;
                        }
                    }
                    Err(e) => {}
                }
            }

            let find = self.find_subsequence(&self.body, s.as_bytes());
            if find.is_none() {
                return false;
            } else {
                return true;
            }
        }
    }

    pub fn get_body_string(&self) -> String {
        let e_type = self.orignal.headers().get("content-encoding");
        let e_type = match e_type {
            Some(s) => {
                let s = match s.to_str() {
                    Ok(o) => o,
                    Err(e) => "",
                };

                s
            }
            None => "",
        };

        if e_type.contains("gzip") {
            let s: &[u8] = &self.body;
            let mut d = GzDecoder::new(s);
            let mut s = String::new();
            d.read_to_string(&mut s).unwrap();
            return s;
        }

        return String::from_utf8_lossy(&self.body).to_string();
    }
}


pub static mut HTTP_LOG: Option<LogHistory> = None;
pub static mut SITE_MAP: Option<SiteMap> = None;
#[derive(Default)]
pub struct LogHistory {
    history: HashMap<u32, Arc<ReqResLog>>,
    last_index: u32,
    lock: Mutex<i32>,
}

fn i_to_m<T>(reference: &T) -> &mut T {
    unsafe {
        let const_ptr = reference as *const T;
        let mut_ptr = const_ptr as *mut T;
        &mut *mut_ptr
    }
}
impl LogHistory {
    fn new() -> Self {
        LogHistory::default()
    }

    pub fn single() -> &'static mut Option<LogHistory> {
        unsafe {
            if HTTP_LOG.is_none() {
                HTTP_LOG = Some(LogHistory::default());
            }
            &mut HTTP_LOG
        }
    }

    pub fn get_size(&self) -> usize {
        let mut ret = 0;
        for vk in &self.history {
            ret += vk.1.get_size();
        }

        return ret;
    }

    pub fn push_log(&mut self, log: ReqResLog) -> Result<u32, STError> {
        let result = self.lock.lock();
        let lock = match result {
            Ok(lock) => lock,
            Err(e) => {
                return Err(STError::new(&e.to_string()));
            }
        };
        self.last_index += 1;
        self.history.insert(self.last_index, Arc::new(log));
        let sitemap = match SiteMap::single() {
            Some(s) => s,
            None => {
                return Err(STError::new("Error to get SiteMap single instance"));
            }
        };
        let _ = sitemap.push_log(self.last_index);
        Ok(self.last_index)
    }

    pub fn remove_log(&mut self, index: u32) {
        self.history.remove(&index);
    }

    pub fn get_log(&self, index: u32) -> Option<&Arc<ReqResLog>> {
        self.history.get(&index)
    }
    
    pub fn set_resp(&mut self, index: u32, resp: LogResponse) -> Result<(), STError>{
        match self.lock.lock() {
            Ok(o) => {}
            Err(e) => {}
        };
        let log = self.history.get(&index);
        let log = match log {
            Some(s) => s,
            None => {
                return Err(STError::new("Not existed log"));
            }
        };
        log.set_resp(resp);
        return Ok(());
    }

    pub fn get_httplog(index: u32) -> Option<&'static Arc<ReqResLog>> {
        let history = LogHistory::single();
        let history = match history {
            Some(h) => h,
            None => {
                return None;
            }
        };

        history.get_log(index)
    }

    pub fn get_req_num(&self) -> usize {
        return self.history.len();
    }

    pub fn get_history(&self) -> &HashMap<u32, Arc<ReqResLog>> {
        &self.history
    }
}

pub struct FoundUrl {
    method  : Method,
    url     : String,
    length  : u32,
    status  : u16,
    c_type  : String
}

impl FoundUrl {
    pub fn new(
        method : Method,
        url    : &str,
        length : u32,
        status : u16,
        c_type : &str
    ) -> Self {
        Self {
            method : method,
            url    : url.to_string(),
            length : length,
            status : status,
            c_type : c_type.to_string(),
        }
    }
}

pub struct Site {
    logs    : Vec<u32>,
    issues  : Vec<Issue>,
    paths   : Vec<String>
}

impl Site {
    pub fn new() -> Self {
        Site {
            logs: Vec::default(),
            issues : Default::default(),
            paths : Default::default()
        }
    }

    pub fn push_httplog(&mut self, index: u32) {
        self.logs.push(index);
    }

    pub fn get_logs(&self) -> &Vec<u32> {
        &self.logs
    }

    pub fn push_issue(&mut self, issue: Issue) {
        self.issues.push(issue);
    }

    pub fn get_issues(&self) -> &Vec<Issue> {
        &self.issues
    }

    pub fn add_paths(&mut self, s: &str) {
        if self.paths.contains(&s.to_string()) {
            return ;
        }
        self.paths.push(s.to_string());
    }

    pub fn get_paths(&self) -> &Vec<String> {
        &self.paths
    }
}
pub struct SiteMap {
    map: HashMap<String, Site>,
}

impl SiteMap {
    pub fn single() -> &'static mut Option<SiteMap> {
        unsafe {
            if SITE_MAP.is_none() {
                SITE_MAP = Some(SiteMap {
                    map: HashMap::default(),
                });
            }
            &mut SITE_MAP
        }
    }

    pub fn push_log(&mut self, index: u32) -> Result<(), STError> {
        let history = LogHistory::single();
        let history = match history {
            Some(h) => h,
            None => {
                return Err(STError::new("Can not get LogHistory single instance"));
            }
        };

        let log = history.get_log(index);
        let log = match log {
            Some(s) => s,
            None => {
                return Err(STError::new("Not exist log"));
            }
        };
        let request = match log.get_request() {
            Some(r) => r,
            None => {
                return Err(STError::new("Can not get request from ReqResLog"));
            }
        };

        let host = request.get_host();
        if self.map.contains_key(&host) == false {
            self.map.insert(host.to_string(), Site::new());
        }

        let site = match self.map.get_mut(&host) {
            Some(s) => s,
            None => {
                return Err(STError::new("Can not get site from Sitemap"));
            }
        };

        site.push_httplog(index);
        Ok(())
    }

    pub fn get_hosts(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    pub fn get_httplogs_by_host(&self, s: &str) -> Option<&Vec<u32>> {
        let site = match self.map.get(s) {
            Some(si) => si,
            None => {
                return None;
            }
        };

        Some(site.get_logs())
    }
    /** `push_issue` Push issue to Site , Don't save the same issues in one site


    ```
    site.push_issue(issue);
    ```
    */
    pub fn push_issue(&mut self, issue: Issue) -> Result<(),STError> {
        let host = issue.get_host();
        if self.map.contains_key(host) == false {
            self.map.insert(host.to_string(), Site::new());
        }

        let site = self.map.get_mut(host);
        let site = match site {
            Some(s) => s,
            None => {
                return Err(STError::new("Not existed site"));
            }
        };
        for iter in &site.issues {
            if iter.get_name().eq(issue.get_name()) {
                
                if iter.get_host().eq(issue.get_host()) {
                    return Ok(());
                }
            }
        }
        site.push_issue(issue);
        return Ok(());
    }

    pub fn get_site(&self, host: &str) -> Option<&Site> {
        self.map.get(host)
    }

    pub fn add_exist_path(&mut self, url: &FoundUrl) {
        let uri = Uri::from_str(&url.url);
        let uri = match uri {
            Ok(o) => o,
            Err(e) => {
                error!("{}", e);
                return ;
            }
        };

        let key: String;
        if uri.port().is_none() {
            let host = match uri.host() {
                Some(s) => s,
                None => {
                    error!("host not found");
                    return ;
                }
            };
            key = format!("{}",host);
        } else {
            let host = match uri.host() {
                Some(s) => s,
                None => {
                    error!("host not found");
                    return ;
                }
            };
            key = format!("{}:{}", host, uri.port().unwrap());
        }
        if self.map.contains_key(&key) == false {
            self.map.insert(key.to_string(), Site::new());
        }
        let site = match self.map.get_mut(&key) {
            Some(s) => s,
            None => {
                error!("Can not get site from Sitemap");
                return ;
            }
        };

        let s = format!("{} {} {} {} {}",
        url.method.as_str().green(), uri.path().yellow(),url.status.to_string().blue(), url.length, url.c_type.green());
        site.add_paths(&s);
    }
}
