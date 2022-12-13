pub mod active;
pub mod passive;

#[derive(Default)]
pub struct Helper {}

pub enum IssueLevel {
    Info,
    Medium,
    HighRisk,
    Critical,
}

pub struct Issue {
    name: String,
    detail: String,
    level: IssueLevel,
    confidence: IssueConfidence,
    httplog: Option<ReqResLog>,
    host: String,
}

pub enum IssueConfidence {
    Suspicious,
    Confirm,
}

impl Issue {
    pub fn new(
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

    pub fn get_httplog(&self) -> &Option<ReqResLog> {
        &self.httplog
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

use std::{collections::HashMap};

use crate::{
    cmd::handlers::SCAN_RECEIVER,
    proxy::log::{ReqResLog, SiteMap},
    utils::STError,
};

#[derive(Debug,Clone)]
pub struct ModuleMeta {
    name: String,
    description: String,
}



impl ModuleMeta {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}
type Args = HashMap<String, String>;
pub trait IActive {
    //Use in proxy mod
    fn passive_run(&self, index: u32) -> Result<Vec<Issue>, STError>;
    //Use in cmd mod
    fn active_run(&self, url: &str, args: Args) -> Result<Vec<Issue>, STError>;

    fn metadata(&self) -> &Option<ModuleMeta>;
}

pub static mut GLOB_POCS: Vec<ModuleMeta> = Vec::<ModuleMeta>::new();
pub static mut WILL_RUN_POCS: Vec<String> = Vec::<String>::new();
pub fn get_modules() -> &'static Vec<ModuleMeta> {
    unsafe {
        &GLOB_POCS
    }
}

pub fn get_will_run_pocs() -> &'static Vec<String> {
    unsafe {
        &WILL_RUN_POCS
    }
}

pub fn push_will_run_poc(name: &str) {
    unsafe {
        WILL_RUN_POCS.push(name.to_string());
    }
}

pub fn remove_will_run_poc(name: &str) {
    unsafe {
        let index = WILL_RUN_POCS.iter().position(|r| r.eq(name));
        let index = match index {
            Some(i) => i,
            None => {
                return ;
            }
        };

        WILL_RUN_POCS.remove(index);
    }
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

