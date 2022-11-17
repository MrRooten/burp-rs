#![allow(dead_code)]
use hudsucker::hyper::{Request, Body, Response};
use std::collections::HashMap;
#[derive(Default)]
pub struct ReqResLog {
    request     : Option<LogRequest>,
    response    : Option<LogResponse>
}

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
pub struct LogResponse {
    orignal     : Response<Body>
}

impl LogResponse {
    pub fn from(res: Response<Body>) -> LogResponse {
        LogResponse { orignal: res }
    }
}

static mut HTTP_LOG: Option<LogHistory> = None;

#[derive(Default)]
pub struct LogHistory {
    history     : HashMap<u32,ReqResLog>,
    last_index  : u32
}

impl LogHistory {
    fn new() -> Self {
        LogHistory::default()
    }

    pub fn single() -> &'static Option<LogHistory> {
        unsafe {
            if HTTP_LOG.is_none() {
                HTTP_LOG = Some(LogHistory::default());
            }
            &HTTP_LOG
        }
    }

    pub fn push_log(&mut self, log: ReqResLog) {
        self.last_index += 1;
        self.history.insert(self.last_index, log);
    }

    pub fn remove_log(&mut self, index: u32) {
        self.history.remove(&index);
    }

    pub fn get_log(&self,index: u32) -> Option<&ReqResLog> {
        self.history.get(&index)
    }
}

pub struct Site {
    logs    : Vec<u32>,
}

pub struct SiteMap {
    map     : HashMap<String,Site>
}