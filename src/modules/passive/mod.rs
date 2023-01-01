
pub mod cookie_match;
pub mod path_match;
pub mod param_inspect;
pub mod js_miner;
pub mod serialize_detect;
use log::error;

use crate::{proxy::log::LogHistory, librs::http::utils::HttpRequest};

use self::cookie_match::CookieMatch;

use super::IPassive;

pub struct PassiveScanner {
    modules     : Vec<Box<(dyn IPassive+'static)>>
}

impl PassiveScanner {
    pub fn new() -> Self {
        let mut ret: Vec<Box<(dyn IPassive + 'static)>> = Vec::default();
        ret.push(Box::new(CookieMatch));
        Self {
            modules : ret
        }
    }

    pub fn passive_scan(&self, index: u32) {
        let log = match LogHistory::get_httplog(index) {
            Some(s) => s,
            None => {
                return ;
            }
        };

        let request = match log.get_request() {
            Some(s) => s,
            None => {
                return ;
            }
        };

        let request = HttpRequest::from_log_request(request);
        let burp = request.to_burp();
        for module in &self.modules {
            let result = match module.run(log, &burp) {
                Ok(o) => {},
                Err(e) => {
                    error!("{}",e);
                    return ;
                }
            };
        }
    }
}