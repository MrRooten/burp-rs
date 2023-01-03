
pub mod cookie_match;
pub mod path_match;
pub mod param_inspect;
pub mod js_miner;
use log::error;

use crate::{proxy::log::LogHistory, librs::http::utils::HttpRequest};

use self::{cookie_match::CookieMatch, param_inspect::ParamInspect, js_miner::JsMiner, path_match::PathMatch};

use super::IPassive;

pub struct PassiveScanner {
    modules     : Vec<Box<(dyn IPassive+'static)>>
}

impl PassiveScanner {
    pub fn new() -> Self {
        let mut ret: Vec<Box<(dyn IPassive + 'static)>> = Vec::default();
        ret.push(Box::new(CookieMatch));
        ret.push(Box::new(ParamInspect));
        ret.push(Box::new(JsMiner));
        ret.push(Box::new(PathMatch));
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
        let params = burp.get_params().unwrap_or(vec![]);
        for module in &self.modules {
            let result = match module.run(log, &burp, &params) {
                Ok(o) => {},
                Err(e) => {
                    error!("{}",e);
                    return ;
                }
            };
        }
    }
}