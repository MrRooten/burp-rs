use colored::Colorize;

use crate::{modules::{get_will_run_pocs, get_modules, push_will_run_poc}, utils::STError, libruby::rb_main::{set_reload, get_running_modules, remove_dead_modules}};

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
        "Push pocs, support wildcard pattern".to_string()
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

pub struct LoadedPocs {
    opts    : CMDOptions
}

impl LoadedPocs {
    pub fn new() -> Self {
        Self { opts: Default::default() }
    }
}

impl CMDProc for LoadedPocs {
    fn get_name(&self) -> &str {
        "loaded_pocs"
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

pub struct Reload {
    opts    : CMDOptions
}

impl Reload {
    pub fn new() -> Self { 
        Reload { opts: Default::default() }
    }
}

impl CMDProc for Reload {
    fn get_name(&self) -> &str {
        "reload"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        set_reload();
        Ok(())
    }

    fn get_detail(&self) -> String {
        "Reload all modules".to_string()
    }

    fn get_help(&self) -> String {
        "reload".to_string()
    }
    
}

pub struct RunningPocs {
    opts    : CMDOptions
}

impl RunningPocs {
    pub fn new() -> Self { 
        RunningPocs { opts: Default::default() }
    }
}

impl CMDProc for RunningPocs {
    fn get_name(&self) -> &str {
        "running_pocs"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        if line.len() >= 2 {
            if line[1].eq("--remove-dead") || line[1].eq("-rd") {
                remove_dead_modules();
            }
        }
        let modules = get_running_modules();
        let modules = match modules {
            Some(s) => s,
            None => {
                return Err(STError::new("RUNNING_MODULES does not initialize..."));
            }
        };

        let mut keys = modules.keys().cloned().collect::<Vec<u32>>();
        keys.sort();
        for i in keys {
            println!("{: >3} {: <20} {} {: >3} {: <10} {: >7}{}" , 
            i,
            modules.get(&i).unwrap().get_name().blue(), 
            modules.get(&i).unwrap().get_starttime().to_rfc2822(), 
            modules.get(&i).unwrap().get_args().to_string().yellow(), 
            modules.get(&i).unwrap().get_state_colored(),
            (modules.get(&i).unwrap().get_cost() as f32 / 1000 as f32) as f32,
            " second".green()
        );
        }

        Ok(())
    }

    fn get_detail(&self) -> String {
        "List Running modules".to_string()
    }

    fn get_help(&self) -> String {
        "reload".to_string()
    }
    
}