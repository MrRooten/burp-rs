use hyper::{Body, Request, Response};
use log::{debug, error};

use crate::utils::{config::get_config, log::can_debug};

pub fn is_capture_req(req: &Request<Body>) -> bool {
    if can_debug() {
        debug!("burp-rs:Send request: {:?}", req);
    }
    true
}

pub fn is_capture_res(res: &Response<Body>) -> bool {
    let config = get_config();
    let content_type = res.headers().get("content-type");
    if can_debug() {
        debug!("burp-rs:Recive Response: {:?}", res);
    }

    match content_type {
        Some(t) => {
            let s = t.to_str();
            match s {
                Ok(o) => {
                    let filter_type = config.get("body_filter.content_type").as_vec();
                    if filter_type.is_none() {
                        if o.contains("text") || o.contains("json") || o.contains("xml") {
                            return true;
                        }
                    }
                    let filter_types = filter_type.unwrap();
                    for i in filter_types {
                        let t = i.as_str().unwrap();
                        if t.starts_with("!") {
                            if o.contains(&t[1..]) {
                                return false;
                            }
                        } else {
                            if o.contains(t) {
                                return true;
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("{}", e);
                }
            }
        }
        None => {}
    };
    let s = res.headers().get("content-length");
    let value = match s {
        Some(v) => v,
        None => return false,
    };

    let max_size = config.get("body_filter.size_max").as_str();
    let value = value.to_str();
    let value = match value {
        Ok(s) => {
            let length = s.parse::<u32>();
            match length {
                Ok(o) => o,
                Err(e) => {
                    return false;
                }
            }
        }
        Err(e) => {
            return false;
        }
    };
    match max_size {
        Some(o) => {
            let ratio: u32;
            if o.ends_with("M") {
                ratio = 1024 * 1024;
            } else if o.ends_with("K") {
                ratio = 1024;
            } else if o.ends_with("B") {
                ratio = 1;
            } else {
                ratio = 1;
            }
            //if empty string, default action
            if o.len() == 0 {
                if value < 1024 * 1024 {
                    return true;
                } else {
                    return false;
                }
            }
            let max = match o[0..o.len() - 1].parse::<u32>() {
                Ok(o) => o,
                Err(e) => {
                    //if parse int error, default action
                    if value < 1024 * 1024 {
                        return true;
                    } else {
                        return false;
                    }
                }
            };

            let max = max * ratio;

            if value <= max {
                return true;
            } else {
                return false;
            }
        }
        None => {
            //if size_max is not set, default action
            if value < 1024 * 1024 {
                return true;
            } else {
                return false;
            }
        }
    };
}
