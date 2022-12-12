use crate::{modules::{get_will_run_pocs, get_modules, push_will_run_poc}, utils::STError};

use super::cmd_handler::{CMDProc, CMDOptions};



pub struct PushPoc {
    opts    : CMDOptions
}

impl PushPoc {
    pub fn new() -> Self {
        PushPoc { opts: Default::default() }
    }
}

impl CMDProc for PushPoc {
    fn get_name(&self) -> &str {
        "push_poc"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        let pocs = &line[1..];
        for poc in pocs {
            push_will_run_poc(poc);
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        "Push pocs".to_string()
    }

    fn get_help(&self) -> String {
        "push_poc ${poc_name}".to_string()
    }
}

pub struct ListPocs {
    opts    : CMDOptions
}

impl ListPocs {
    pub fn new() -> Self {
        ListPocs { opts: Default::default() }
    }
}

impl CMDProc for ListPocs {
    fn get_name(&self) -> &str {
        "list_pocs"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        let modules = get_modules();
        for module in modules {
            println!("{}",module.get_name());
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        "list pocs".to_string()
    }

    fn get_help(&self) -> String {
        "list_pocs".to_string()
    }
}

pub struct RunPocs {
    opts    : CMDOptions
}

impl RunPocs {
    pub fn new() -> Self {
        Self { opts: Default::default() }
    }
}

impl CMDProc for RunPocs {
    fn get_name(&self) -> &str {
        "run_pocs"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        let pocs = get_will_run_pocs();
        for poc in pocs {
            println!("{}",poc);
        }

        Ok(())
    }

    fn get_detail(&self) -> String {
        "List running pocs".to_string()
    }

    fn get_help(&self) -> String {
        "run_pocs".to_string()
    }
}

