use log::error;

use crate::utils::config::get_config;

use super::cmd_handler::{CMDOptions, CMDProc};

pub struct ReloadConfig {
    opts        : CMDOptions
}

impl ReloadConfig {
    fn new() -> Self {
        Self {
            opts : Default::default()
        }
    }
}

impl CMDProc for ReloadConfig {
    fn get_name(&self) -> &str {
        "reload_config"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), crate::utils::STError> {
        let config = get_config();
        match config.update() {
            Ok(o) => {},
            Err(e) => {
                error!("{}",e);
            }
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        "reload config".to_string()
    }

    fn get_help(&self) -> String {
        "reload_config".to_string()
    }
}