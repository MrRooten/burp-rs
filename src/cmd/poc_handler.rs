use colored::Colorize;

use crate::{modules::{get_will_run_mods, get_modules_meta, push_will_run_mod, remove_loaded_mod, ModuleType}, utils::STError, scanner::{set_reload, remove_dead_modules, get_running_modules}};

use super::cmd_handler::{CMDProc, CMDOptions};



pub struct PushMod {
    opts    : CMDOptions
}

impl Default for PushMod {
    fn default() -> Self {
        Self::new()
    }
}

impl PushMod {
    pub fn new() -> Self {
        PushMod { opts: Default::default() }
    }
}

impl CMDProc for PushMod {
    fn get_name(&self) -> &str {
        "push_mod"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        let pocs = &line[1..];
        for poc in pocs {
            push_will_run_mod(poc);
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        "Push pocs, support wildcard pattern".to_string()
    }

    fn get_help(&self) -> String {
        "push_mod ${poc_name}".to_string()
    }
}

pub struct ListMods {
    opts    : CMDOptions
}

impl Default for ListMods {
    fn default() -> Self {
        Self::new()
    }
}

impl ListMods {
    pub fn new() -> Self {
        ListMods { opts: Default::default() }
    }
}

impl CMDProc for ListMods {
    fn get_name(&self) -> &str {
        "list_mods"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        let modules = get_modules_meta();
        for module in modules {
            let s = if module.get_type().eq(&ModuleType::RustModule) {
                "RustModule".yellow()
            } else {
                "RubyModule".red()
            };
            println!("{: <20}: {: <20}  {}",module.get_name().green(), s, module.get_description());
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        "list mods".to_string()
    }

    fn get_help(&self) -> String {
        "list_mods".to_string()
    }
}

pub struct LoadedMods {
    opts    : CMDOptions
}

impl Default for LoadedMods {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadedMods {
    pub fn new() -> Self {
        Self { opts: Default::default() }
    }
}

impl CMDProc for LoadedMods {
    fn get_name(&self) -> &str {
        "loaded_mods"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        let pocs = get_will_run_mods();
        for module in pocs {

            let s = if module.get_type().eq(&ModuleType::RustModule) {
                "RustModule".yellow()
            } else {
                "RubyModule".red()
            };
            println!("{: <20}: {: <20}  {}",module.get_name().green(), s, module.get_description());
        }

        Ok(())
    }

    fn get_detail(&self) -> String {
        "List mods that loaded task".to_string()
    }

    fn get_help(&self) -> String {
        "run_mods".to_string()
    }
}

pub struct Reload {
    opts    : CMDOptions
}

impl Default for Reload {
    fn default() -> Self {
        Self::new()
    }
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
        "Reload all modules that reloadable (RubyModule can be reloaded)".to_string()
    }

    fn get_help(&self) -> String {
        "reload".to_string()
    }
    
}

pub struct RunningMods {
    opts    : CMDOptions
}

impl Default for RunningMods {
    fn default() -> Self {
        Self::new()
    }
}

impl RunningMods {
    pub fn new() -> Self { 
        RunningMods { opts: Default::default() }
    }
}

impl CMDProc for RunningMods {
    fn get_name(&self) -> &str {
        "running_mods"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        if line.len() >= 2 && (line[1].eq("--remove-dead") || line[1].eq("-rd")) {
            remove_dead_modules();
        }
        let modules = get_running_modules();
        let modules = match modules {
            Some(s) => s,
            None => {
                return Err(STError::new("RUNNING_MODULES does not initialize..."));
            }
        };

        let mut keys = modules.keys().cloned().collect::<Vec<i32>>();
        keys.sort();
        for i in keys {
            if modules.get(&i).is_none() {
                continue;
            }
            println!("{: >3} {: <20} {} {: >3} {: <10} {: >7}{}" , 
            i,
            modules.get(&i).unwrap().get_name().bright_blue(), 
            modules.get(&i).unwrap().get_starttime().to_rfc2822(), 
            modules.get(&i).unwrap().get_args().to_string().yellow(), 
            modules.get(&i).unwrap().get_state_colored(),
            (modules.get(&i).unwrap().get_cost() as f32 / 1000_f32) as f32,
            " second".green()
        );
        }

        Ok(())
    }

    fn get_detail(&self) -> String {
        "List Running modules".to_string()
    }

    fn get_help(&self) -> String {
        "running_mods".to_string()
    }
    
}

pub struct RemoveMod {
    opts    : CMDOptions
}

impl Default for RemoveMod {
    fn default() -> Self {
        Self::new()
    }
}

impl RemoveMod {
    pub fn new() -> RemoveMod {
        Self { opts: Default::default() }
    }
}

impl CMDProc for RemoveMod {
    fn get_name(&self) -> &str {
        "remove_mod"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        let pocs = &line[1..];
        for poc in pocs {
            remove_loaded_mod(poc);
        }

        Ok(())
    }

    fn get_detail(&self) -> String {
        "Remove poc that loaded".to_string()
    }

    fn get_help(&self) -> String {
        "remove_mod ${wilcard_pattern}".to_string()
    }
}