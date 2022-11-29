use std::process;

use hyper::StatusCode;

use crate::{proxy::log::LogHistory, utils::{STError, log::logs}};

use super::cmd_handler::*;

#[derive(Default)]
pub struct Helper {
    name    : String,
    opts    : CMDOptions,
    
}

impl CMDProc for Helper {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_opts(&self) -> &CMDOptions {
        return &self.opts;
    }

    fn process(&self, line: &Vec<&str>) -> Result<(),crate::utils::STError> {
        println!("help");
        Ok(())
    }

    
}

impl Helper {
    pub fn new() -> Self {
        Self {
            name: "help".to_string(),
            opts: Default::default()
        }
    }
}
#[derive(Default)]
pub struct Exit {
    name    : String,
    opts    : CMDOptions,
    
}

impl CMDProc for Exit {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_opts(&self) -> &CMDOptions {
        return &self.opts;
    }

    fn process(&self, line: &Vec<&str>) -> Result<(),crate::utils::STError> {
        process::exit(0);
    }

    
}

impl Exit {
    pub fn new() -> Self {
        Self {
            name: "exit".to_string(),
            opts: Default::default()
        }
    }
}

pub struct ProxyLogInfo {
    name    : String,
    opts    : CMDOptions
}

impl CMDProc for ProxyLogInfo {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_opts(&self) -> &CMDOptions {
        return &self.opts;
    }

    fn process(&self, line: &Vec<&str>) -> Result<(),crate::utils::STError> {
        let history = LogHistory::single();
        let history = match history {
            Some(s) => s,
            None => {
                return Err(STError::new("Error to get LogHistory"));
            }
        };

        println!("Proxy Request:{}",history.get_req_num());
        Ok(())
    }

    
}

impl ProxyLogInfo {
    pub fn new() -> Self {
        Self {
            name: "proxylog_info".to_string(),
            opts: Default::default(),
        }
    }
}

pub struct ListHistory {
    name    : String,
    opts    : CMDOptions
}

impl ListHistory {
    pub fn new() -> Self {
        Self {
            name: "list_history".to_string(),
            opts: Default::default(),
        }
    }
}

impl CMDProc for ListHistory {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(),STError> {
        let history = LogHistory::single();
        let history = match history {
            Some(s) => s,
            None => {
                return Err(STError::new("Error to get LogHistory"));
            }
        };

        let history = history.get_history();
        let mut keys = history.keys().collect::<Vec<&u32>>();
        keys.sort();
        for key in keys {
            let request = history.get(key).unwrap().get_request().unwrap();
            let response = history.get(key).unwrap().get_response();
            let status = match response {
                Some(r) => r.get_status(),
                None => StatusCode::GONE
            };
            let size = match response {
                Some(r) => r.get_size(),
                None => 0
            };
            println!("{} {} {} {}",key,request.get_url(),status,size);
        }
        Ok(())
    }
}

pub struct DebugLog {
    name    : String,
    opts    : CMDOptions
}

impl DebugLog {
    pub fn new() -> Self {
        Self {
            name: "debug_log".to_string(),
            opts: Default::default(),
        }
    }
}

impl CMDProc for DebugLog {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        unsafe {
            for log in &logs {
                println!("{}", log);
            }
        }

        Ok(())
    }
}