pub mod body_match;
pub mod cookie_match;
pub mod js_miner;
pub mod param_inspect;
pub mod path_match;
use log::error;

use crate::{librs::http::utils::HttpRequest, proxy::log::LogHistory};

use self::{
    js_miner::JsMiner, param_inspect::ParamInspect,
    path_match::PathMatch,
};

use super::IPassive;

pub struct PassiveScanner {
    modules: Vec<Box<(dyn IPassive + 'static)>>,
}

impl PassiveScanner {
    pub fn new() -> Self {
        let mut ret: Vec<Box<(dyn IPassive + 'static)>> = Vec::default();
        //ret.push(Box::new(CookieMatch));
        ret.push(Box::new(ParamInspect));
        ret.push(Box::new(JsMiner));
        ret.push(Box::new(PathMatch));
        Self { modules: ret }
    }

    pub fn passive_scan(&self, index: u32) {
        let log = match LogHistory::get_httplog(index) {
            Some(s) => s,
            None => {
                return;
            }
        };

        let request = log.get_request();

        let request = HttpRequest::from_log_request(request);
        let burp = request.to_burp();
        let params = burp.get_params().unwrap_or(vec![]);
        let body = String::new();
        for module in &self.modules {
            let result = match module.run(log, &burp, &params) {
                Ok(o) => {}
                Err(e) => {
                    error!("{}", e);
                    return;
                }
            };
        }
    }
}
