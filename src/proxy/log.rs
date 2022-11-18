#![allow(dead_code)]
use hudsucker::hyper::{Request, Body, Response};
use hyper::Version;
use url::Url;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::utils::STError;
#[derive(Default)]
pub struct ReqResLog {
    request     : Option<LogRequest>,
    response    : Option<LogResponse>
}

impl ReqResLog {
    pub fn new(req: LogRequest) -> Self {
        ReqResLog {
            request  : Some(req),
            response : None
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
}

#[derive(Debug)]
pub struct LogRequest {
    orignal     : Request<Body>
}

impl LogRequest {
    pub fn from(req: Request<Body>) -> LogRequest {
        LogRequest {
            orignal : req
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


}

#[derive(Debug)]
pub struct LogResponse {
    orignal     : Response<Body>
}

impl LogResponse {
    pub fn from(res: Response<Body>) -> LogResponse {
        LogResponse { orignal: res }
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

    pub fn push_log(&mut self, log: ReqResLog) -> u32 {
        self.lock.lock();
        self.last_index += 1;
        self.history.insert(self.last_index, log);
        let sitemap = match SiteMap::single() {
            Some(s) => s,
            None => {
                return self.last_index;
            }
        };
        sitemap.push(self.last_index);
        self.last_index
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