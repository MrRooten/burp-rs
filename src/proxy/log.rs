#![allow(dead_code)]
use chrono::{Utc, DateTime};
use hudsucker::hyper::{Body, Response};
use hyper::body::Bytes;
use hyper::{StatusCode, http};
use url::Url;
use std::collections::HashMap;
use std::sync::Mutex;
use http::Request;
use crate::librs::http::utils::HttpResponse;
use crate::st_error;
use crate::utils::STError;



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
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct LogResponse {
    orignal     : Response<Body>,
    body        : Bytes
}

impl LogResponse {
    pub fn from(res: Response<Body>, body: Bytes) -> LogResponse {
        LogResponse { orignal: res ,body: body}
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
    pub fn to_string() -> String {
        unimplemented!()
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
        };
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
        self.lock.lock();
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
}