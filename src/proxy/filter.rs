use hyper::{Request, Body, Response};

use super::log::{LogRequest, LogResponse};

pub fn is_capture_req(req: &LogRequest) -> bool {
    true
}

pub fn is_capture_res(res: &LogResponse) -> bool {
    true
}