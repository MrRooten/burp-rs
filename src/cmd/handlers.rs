use crate::{proxy::log::LogHistory, utils::STError};

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
