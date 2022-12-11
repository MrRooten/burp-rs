pub mod active;
pub mod passive;

#[derive(Default)]
pub struct Helper {}

enum IssueLevel {
    Info,
}

pub struct Issue {
    name: String,
    detail: String,
    level: IssueLevel,
    confidence: IssueConfidence,
    httplog: Option<ReqResLog>,
    host: String,
}

enum IssueConfidence {
    Confirm,
}

impl Issue {
    fn new(
        name: &str,
        level: IssueLevel,
        detail: &str,
        confidance: IssueConfidence,
        host: &str,
    ) -> Issue {
        Self {
            name: name.to_string(),
            detail: detail.to_string(),
            level,
            confidence: confidance,
            httplog: None,
            host: host.to_string(),
        }
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    /** `add_issue` Push issue to Site of Sitemap index by hostname


    ```
    Issue::add_issue(issue, httplog);
    ```
    */
    pub fn add_issue(mut issue: Issue, httplog: &ReqResLog) {
        issue.set_httplog(httplog);
        let sitemap = SiteMap::single();
        let sitemap = match sitemap {
            Some(s) => s,
            None => {
                return;
            }
        };

        sitemap.push_issue(issue);
    }

    pub fn get_issues() -> Vec<&'static Issue> {
        let mut ret = Vec::default();
        let sitemap = SiteMap::single();
        let sitemap = match sitemap {
            Some(s) => s,
            None => {
                return Default::default();
            }
        };

        let hosts = sitemap.get_hosts();
        for host in hosts {
            let site = sitemap.get_site(&host);
            let site = match site {
                Some(s) => s,
                None => {
                    continue;
                }
            };

            let issues = site.get_issues();
            for issue in issues {
                ret.push(issue);
            }
        }
        ret
    }

    pub fn set_httplog(&mut self, httplog: &ReqResLog) {
        self.httplog = httplog.clone();
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_detail(&self) -> &str {
        &self.detail
    }

    fn get_level(&self) -> &IssueLevel {
        &self.level
    }

    fn get_confidence(&self) -> &IssueConfidence {
        &self.confidence
    }

    pub fn get_httplog(&self) -> Option<&ReqResLog> {
        None
    }

    pub fn get_url(&self) -> String {
        let httplog = match &self.httplog {
            Some(s) => s,
            None => {
                return "".to_string();
            }
        };

        let request = match httplog.get_request() {
            Some(s) => s,
            None => {
                return "".to_string();
            }
        };

        return request.get_url();
    }
}

pub trait IPassive {
    fn run(&self, index: u32) -> Result<(), STError>;

    fn name(&self) -> String;

    fn help(&self) -> Helper;
}

use std::collections::HashMap;

use crate::{
    cmd::handlers::SCAN_RECEIVER,
    proxy::log::{ReqResLog, SiteMap},
    utils::STError,
};

#[derive(Debug)]
pub struct ModuleMeta {
    name: String,
    description: String,
}

type Args = HashMap<String, String>;
pub trait IActive {
    //Use in proxy mod
    fn passive_run(&self, index: u32) -> Result<Vec<Issue>, STError>;
    //Use in cmd mod
    fn active_run(&self, url: &str, args: Args) -> Result<Vec<Issue>, STError>;

    fn metadata(&self) -> Option<ModuleMeta>;
}

pub fn get_next_to_scan() -> u32 {
    unsafe {
        let receiver = &mut SCAN_RECEIVER;
        let receiver = match receiver {
            Some(o) => o,
            None => {
                //Httplog does not have 0 index
                panic!("Receiver")
            }
        };

        return receiver.recv().unwrap();
    }
}
