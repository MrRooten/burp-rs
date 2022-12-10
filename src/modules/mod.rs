pub mod active;
pub mod passive;

#[derive(Default)]
pub struct Helper {}

enum IssueLevel {
    Info
}

pub static mut ALL_ISSUES: Vec<Issue> = Vec::new();
pub struct Issue {
    name: String,
    detail: String,
    level: IssueLevel,
    confidence: IssueConfidence,
    httplog: Option<ReqResLog>,
}

enum IssueConfidence {
    Confirm
}

impl Issue {
    fn new(
        name: &str,
        level: IssueLevel,
        detail: &str,
        confidance: IssueConfidence,
        httplog: &ReqResLog
    ) -> Issue {
        Self {
            name: name.to_string(),
            detail: detail.to_string(),
            level,
            confidence: confidance,
            httplog: httplog.clone(),
        }
    }

    pub fn add_issue(issue: Issue) {
        unsafe {
            for iter in &ALL_ISSUES {
                if iter.get_name().eq(issue.get_name()) {
                    let iter_host = iter.get_httplog().unwrap().get_host();
                    let issue_host = issue.get_httplog().unwrap().get_host();
                    if iter_host.eq(&issue_host) {
                        return ;
                    }
                }
            }

            ALL_ISSUES.push(issue);
        }
    }

    pub fn get_issues() -> &'static Vec<Issue> {
        unsafe {
            &ALL_ISSUES
        }
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
}

pub trait IPassive {
    fn run(&self, index: u32) -> Result<(), STError>;

    fn name(&self) -> String;

    fn help(&self) -> Helper;
}

use std::{collections::HashMap};

use crate::{cmd::handlers::{SCAN_RECEIVER}, proxy::log::ReqResLog, utils::STError};

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
            Some(o) => {
                o
            },
            None => {
                //Httplog does not have 0 index
                panic!("Receiver")
            },
        };


        return receiver.recv().unwrap();
    }
}
