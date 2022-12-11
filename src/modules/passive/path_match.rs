use std::str::FromStr;

use hyper::Uri;
use url::Url;

use crate::{
    modules::{IPassive, Issue},
    proxy::log::LogHistory,
    utils::STError,
};

pub struct PathMatch;

fn solr(s: &Uri) -> Option<Issue> {
    unimplemented!()
}
impl IPassive for PathMatch {
    fn run(&self, index: u32) -> Result<(), crate::utils::STError> {
        let log = LogHistory::get_httplog(index);
        let log = match log {
            Some(o) => o,
            None => {
                return Err(STError::new("Not found history log"));
            }
        };
        
        let request = match log.get_request() {
            Some(r) => r,
            None => {
                return Err(STError::new("Not found history log request"));
            }
        };
        let url = request.get_url();
        let url = match Url::from_str(&url) {
            Ok(o) => o,
            Err(e) => {
                let msg = format!("{:?}",e);
                return Err(STError::new(&msg));
            }
        };

        unimplemented!()
    }

    fn name(&self) -> String {
        return "PathMatch".to_string();
    }

    fn help(&self) -> crate::modules::Helper {
        todo!()
    }
}
