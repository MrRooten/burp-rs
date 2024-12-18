use std::{str::FromStr, sync::Arc};

use hyper::Uri;
use url::Url;

use crate::{
    modules::{IPassive, Issue, IssueLevel},
    proxy::log::{ReqResLog},
    utils::STError, librs::http::utils::{BurpRequest, BurpParam},
};

pub struct PathMatch;

fn solr(s: &Uri) -> Option<Issue> {
    if s.path().contains("/solr/") {
        let issue = Issue::new(
            "Solr path match",
            IssueLevel::Info,
            "path contains /solr/",
            crate::modules::IssueConfidence::Confirm,
            ""
        );

        return Some(issue);
    }

    None
}


impl IPassive for PathMatch {
    fn run(&self, log: &Arc<ReqResLog>, burp: &BurpRequest, params: &Vec<BurpParam>) -> Result<(), crate::utils::STError> {
        let request = log.get_request();
        let url = request.get_url();
        let url = match Url::from_str(&url) {
            Ok(o) => o,
            Err(e) => {
                let msg = format!("{:?}",e);
                return Err(STError::new(&msg));
            }
        };

        Ok(())
    }

    fn name(&self) -> String {
        "PathMatch".to_string()
    }

    fn help(&self) -> crate::modules::Helper {
        todo!()
    }
}
