use hyper::{Body, Request, Response};
use log::{debug};

use crate::{utils::log::{can_debug}};



pub fn is_capture_req(req: &Request<Body>) -> bool {
    if can_debug() {
        debug!("burp-rs:Send request: {:?}", req);
    }
    true
}

pub fn is_capture_res(res: &Response<Body>) -> bool {
    let content_type = res.headers().get("content-type");
    if can_debug() {
        debug!("burp-rs:Recive Response: {:?}", res);
    }

    match content_type {
        Some(t) => {
            let s = t.to_str();
            match s {
                Ok(o) => {
                    if o.contains("text") || o.contains("json") || o.contains("xml") {
                        return true;
                    }
                }
                Err(e) => {}
            }
        }
        None => {}
    };
    let s = res.headers().get("content-length");
    let value = match s {
        Some(v) => v,
        None => return false,
    };

    let value = value.to_str();
    let value = match value {
        Ok(s) => {
            let length = s.parse::<u32>();
            match length {
                Ok(o) => {
                    if o < 1024 * 1024 {
                        return true;
                    } else {
                        return false;
                    }
                }
                Err(e) => {}
            }
        }
        Err(e) => {
            return false;
        }
    };

    return false;
}
