#![allow(dead_code)]
use hudsucker::hyper::{Request, Body, Response};
use std::collections::HashMap;
use std::sync::Mutex;
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
}

pub struct Site {
    logs    : Vec<u32>,
}

pub struct SiteMap {
    map     : HashMap<String,Site>
}