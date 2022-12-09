pub mod active;
pub mod passive;

#[derive(Default)]
pub struct Helper {}

enum IssueLevel {}
pub struct Issue {
    name: String,
    detail: String,
    level: IssueLevel,
    confidence: IssueConfidence,
    httplog: Option<ReqResLog>,
}

enum IssueConfidence {}

impl Issue {
    fn new(
        name: &str,
        level: IssueLevel,
        detail: &str,
        confidance: IssueConfidence,
        httplog: &ReqResLog,
    ) -> Issue {
        Self {
            name: name.to_string(),
            detail: detail.to_string(),
            level,
            confidence: confidance,
            httplog: None,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_detail(&self) -> &str {
        &self.detail
    }

    fn get_level(&self) -> &IssueLevel {
        &self.level
    }

    fn get_confidence(&self) -> &IssueConfidence {
        &self.confidence
    }

    fn get_httplog(&self) -> Option<&ReqResLog> {
        None
    }
}

pub trait IPassive {
    fn run(&self, index: u32) -> Result<Vec<Issue>, STError>;

    fn name(&self) -> String;

    fn help(&self) -> Helper;
}

use std::{collections::HashMap, sync::mpsc};

use crate::{cmd::handlers::{TO_SCAN_QUEUE, SCAN_RECEIVER}, proxy::log::ReqResLog, utils::STError};

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
