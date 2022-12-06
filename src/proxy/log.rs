#![allow(dead_code)]
use chrono::{Utc, DateTime};
use colored::Colorize;
use flate2::read::GzDecoder;
use hudsucker::hyper::{Body, Response};
use hyper::body::Bytes;
use hyper::{StatusCode, http, Version};
use url::Url;
use std::collections::HashMap;
use std::io::Read;
use std::sync::Mutex;
use http::Request;
use crate::librs::http::utils::HttpResponse;
use crate::librs::object::object::IObject;
use crate::utils::STError;
use crate::utils::utils::{tidy_html, highlighter};


pub struct ReqResLog {
    request     : Option<LogRequest>,
    response    : Option<LogResponse>,
    record_t    : DateTime<Utc>
}

impl ReqResLog {
    pub fn from_http_response(response: &HttpResponse) -> ReqResLog {
        unimplemented!()
    }

    pub fn new(req: LogRequest) -> Self {
        ReqResLog {
            request  : Some(req),
            response : None,
            record_t : Utc::now()
        }
    }

    pub fn set_resp(&mut self, resp: LogResponse) {
        self.response = Some(resp);
    }

    pub fn get_request(&self) -> Option<&LogRequest> {
        match &self.request {
            Some(r) => {
                return Some(r);
            },
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
            },
            None => {
                return None;
            }
        }
    }

    pub fn clone(&self) -> Option<ReqResLog>{
        let s = match &self.request {
            Some(s) => s,
            None => {
                return None;
            }
        };

        let mut log = ReqResLog::new(s.clone());
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

#[derive(Debug)]
pub struct LogRequest {
    orignal     : Request<Body>,
    body        : Bytes,
    record_t    : DateTime<Utc>
}

impl IObject for LogRequest {
    fn get_object(&self, path: &str) -> Option<String> {
        let path = path.trim();
        if path.len() == 0 {
            let s = format!("{:?}",vec!["url","headers","body","method","protocol"]);
            return Some(s);
        }
        let spl = path.split(".").collect::<Vec<&str>>();
        if spl.len() == 0 {
            let s = format!("{:?}",vec!["url","headers","body","method","protocol"]);
            return Some(s);
        }

        let s1 = spl[0];
        if s1.eq("uri") || s1.eq("url") {
            let s = self.orignal.uri().to_string();
            return Some(s);
        }
        else if s1.eq("headers") {
            if spl.len() == 1 {
                let s = format!("{:?}",self.orignal.headers());
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

        }
        else if s1.eq("body") {
            return Some(String::from_utf8_lossy(&self.body).to_string());
        }
        else if s1.eq("method") {
            return Some(self.orignal.method().to_string())
        }
        else if s1.eq("protocol") {
            let s = format!("{:?}",self.orignal.version());
            return Some(s);
        }
        return None;
    }
}

impl LogRequest {
    pub fn from(req: Request<Body>,body: Bytes) -> LogRequest {
        LogRequest {
            orignal : req,
            body    : body,
            record_t: Utc::now()
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
            record_t : self.record_t.clone()
        }
    }

    pub fn get_url(&self) -> String {
        self.orignal.uri().to_string()
    }

    pub fn get_host(&self) -> String {
        let s_url = self.get_url();
        let url = Url::parse(&s_url).unwrap();
        let s = url.host_str().expect("msg").to_string();
        return s;
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        let value = self.orignal.headers().get(key);
        match value {
            Some(v) => {
                return Some(String::from_utf8_lossy(v.as_bytes()).to_string());
            },
            None => {
                return None;
            }
        };
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
        }
        else if self.orignal.version() == Version::HTTP_10 {
            ret.push_str("HTTP/0.9");
        }
        else if self.orignal.version() == Version::HTTP_2 {
            ret.push_str("HTTP/2");
        }
        else if self.orignal.version() == Version::HTTP_3 {
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
    orignal     : Response<Body>,
    body        : Bytes,
    c_type      : String
}

impl LogResponse {
    pub fn from(res: Response<Body>, body: Bytes) -> LogResponse {
        let content_type = res.headers().get("content-type");
        let content_type = match content_type {
            Some(c) => c.to_str().unwrap().to_string(),
            None => {
                return LogResponse { orignal: res ,body: body, c_type: "".to_string()};
            }
        };
        LogResponse { orignal: res ,body: body, c_type: content_type}
    }

    pub fn get_header(&self, key: &str) -> Option<String> {
        let value = self.orignal.headers().get(key);
        match value {
            Some(v) => {
                return Some(String::from_utf8_lossy(v.as_bytes()).to_string());
            },
            None => {
                return None;
            }
        };
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
            let key = kv.0.as_str().blue();
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
                    let (s,_) = prettify_js::prettyprint(&self.get_body_string());
                    s
                } else if c.contains("javascript") {
                    let (s,_) = prettify_js::prettyprint(&self.get_body_string());
                    s
                }
                else {
                    self.get_body_string()
                }
            },
            None => self.get_body_string()
        };
        ret.push_str(&highlighter(&body));
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
            c_type: self.c_type.clone()
        };
    }
    fn find_subsequence(&self, haystack: &[u8], needle: &[u8]) -> Option<usize> {
        haystack.windows(needle.len()).position(|window| window == needle)
    }
    pub fn contains(&self, s: &str, ignore_case: bool) -> bool {
        if ignore_case {
            if self.orignal.status().to_string().to_lowercase().contains(&s.to_lowercase()) {
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
                    }, 
                    Err(e) => {

                    }
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
                    }, 
                    Err(e) => {

                    }
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
                    Ok(o) => {
                        o
                    },
                    Err(e) => {
                        ""
                    }
                };

                s
            }
            None => {
                ""
            }
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
    history     : HashMap<u32,ReqResLog>,
    last_index  : u32,
    lock        : Mutex<i32>
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

    pub fn push_log(&mut self, log: ReqResLog) -> Result<u32,STError> {
        let result = self.lock.lock();
        let lock = match result {
            Ok(lock) => lock,
            Err(e) => {
                return Err(STError::new(&e.to_string()));
            }
        };
        self.last_index += 1;
        self.history.insert(self.last_index, log);
        let sitemap = match SiteMap::single() {
            Some(s) => s,
            None => {
                return Err(STError::new("Error to get SiteMap single instance"));
            }
        };
        let _ = sitemap.push(self.last_index);
        Ok(self.last_index)
    }

    pub fn remove_log(&mut self, index: u32) {
        self.history.remove(&index);
    }

    pub fn get_log(&self,index: u32) -> Option<&ReqResLog> {
        self.history.get(&index)
    }

    pub fn set_resp(&mut self, index: u32, resp: LogResponse) {
        match self.lock.lock() {
            Ok(o) => {

            },
            Err(e) => {

            }
        };
        let log = self.history.get_mut(&index).unwrap();
        log.set_resp(resp);
    }

    pub fn get_httplog(index: u32) -> Option<&'static ReqResLog> {
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

    pub fn get_history(&self) -> &HashMap<u32,ReqResLog> {
        &self.history
    }
}

pub struct Site {
    logs    : Vec<u32>,
}

impl Site {
    pub fn new() -> Self {
        Site {
            logs : Vec::default()
        }
    }

    pub fn push_httplog(&mut self, index: u32) {
        self.logs.push(index);
    }

    pub fn get_logs(&self) -> &Vec<u32> {
        &self.logs
    }
}
pub struct SiteMap {
    map     : HashMap<String,Site>
}

impl SiteMap {
    pub fn single() -> &'static mut Option<SiteMap> {
        unsafe {
            if SITE_MAP.is_none() {
                SITE_MAP = Some(SiteMap { map: HashMap::default() });
            }
            &mut SITE_MAP
        }
    }

    pub fn push(&mut self, index: u32) -> Result<(),STError> {
        let history = LogHistory::single();
        let history = match history {
            Some(h) => h,
            None => {
                return Err(STError::new("Can not get LogHistory single instance"));
            }
        };

        let log = history.get_log(index).unwrap();
        let request = match log.get_request() {
            Some(r) => r,
            None => {
                return Err(STError::new("Can not get request from ReqResLog"));
            }
        };

        let host = request.get_host();
        if self.map.contains_key(&host) == false {
            self.map.insert(host.to_string(),Site::new());
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
}