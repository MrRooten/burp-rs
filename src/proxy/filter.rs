use hyper::{Request, Body, Response};

use super::log::{LogRequest, LogResponse};

pub fn is_capture_req(req: &Request<Body>) -> bool {
    true
}

pub fn is_capture_res(res: &Response<Body>) -> bool {
    true
}