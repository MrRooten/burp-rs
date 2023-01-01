use std::sync::Arc;

use crate::{
    modules::{IPassive, Issue, IssueConfidence, IssueLevel},
    proxy::log::{ReqResLog},
    utils::STError, librs::http::utils::BurpRequest,
};

pub struct CookieMatch;

impl IPassive for CookieMatch {
    fn run(&self, log: &Arc<ReqResLog>, burp: &BurpRequest) -> Result<(), crate::utils::STError> {
        let request = match log.get_request() {
            Some(r) => r,
            None => {
                return Err(STError::new("Not found history log request"));
            }
        };

        let header = request.get_header("cookie");
        let header = match header {
            Some(o) => {
                o
            }
            None => {
                "".to_string()
            }
        };
        if header.to_lowercase().contains("rememberme") {
            let issue = Issue::new(
                "Shiro framework",
                IssueLevel::Info,
                "using remember-me in cookie",
                IssueConfidence::Confirm,
                &log.get_host()
            );
            Issue::add_issue(issue, log);
        }
        let response = match log.get_response() {
            Some(r) => r,
            None => {
                return Err(STError::new("Not found history log request"));
            }
        };
        let header = response.get_header("set-cookie");
        let header = match header {
            Some(o) => {
                o
            }
            None => {
                "".to_string()
            }
        };

        if header.to_lowercase().contains("rememberme") {
            let issue = Issue::new(
                "Shiro framework",
                IssueLevel::Info,
                "using remember-me in set-cookie",
                IssueConfidence::Confirm,
                &log.get_host()
            );
            Issue::add_issue(issue, log);
        }

        Ok(())
    }

    fn name(&self) -> String {
        return "CookieMatch".to_string();
    }

    fn help(&self) -> crate::modules::Helper {
        todo!()
    }
}
