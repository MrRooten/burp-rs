use std::{collections::HashMap, time::{UNIX_EPOCH, SystemTime}};

use colored::Colorize;
use log::{info, error};

use crate::{proxy::log::{SiteMap, LogHistory}, st_error, utils::STError, scanner::{get_modules, RunningModuleWrapper, add_running_modules, RunningState, remove_running_modules}, modules::IActive};

use super::{
    cmd_handler::{CMDOptions, CMDProc},
    handlers::TO_SCAN_QUEUE,
};

pub struct Push {
    opts: CMDOptions,
}

impl CMDProc for Push {
    fn get_name(&self) -> &str {
        "push"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        if line.len() <= 1 {
            let s = format!("{} ${{num}}", self.get_name());
            return Err(STError::new(&s));
        }
        if line[1].starts_with("host:") {
            let host = line[1][5..].to_string();
            let map = match SiteMap::single() {
                Some(s) => s,
                None => {
                    return Err(STError::new("Can not get Sitemap Single instance"));
                }
            };

            let s = map.get_site(&host);
            let site = match s {
                Some(o) => o,
                None => {
                    return Err(STError::new("Doesn't match site"));
                }
            };

            let logs = site.get_logs();
            for log in logs {
                unsafe {
                    TO_SCAN_QUEUE.push(log.clone());
                }
            }
        } else if line[1].to_string().parse::<u32>().is_ok() {
            let index = line[1].to_string().parse::<u32>();
            let index = match index {
                Ok(o) => o,
                Err(e) => {
                    return Err(st_error!(e));
                }
            };
            unsafe {
                TO_SCAN_QUEUE.push(index);
            }
        }

        Ok(())
    }

    fn get_detail(&self) -> String {
        format!(r#"push history log to scan queue, like: 
            `{} 1 2 3` 
                push index eq 1 2 3 to scan queue or 
            `{} host:google.com` 
                push host is google indexes to scan queue"#,"push".green(), "push".green())
    }

    fn get_help(&self) -> String {
        "push ${index}".to_string()
    }
}

impl Push {
    pub fn new() -> Self {
        Self {
            opts: Default::default(),
        }
    }
}

pub struct ListTarget {
    opts: CMDOptions,
}

impl ListTarget {
    pub fn new() -> Self {
        Self {
            opts: Default::default(),
        }
    }
}

impl CMDProc for ListTarget {
    fn get_name(&self) -> &str {
        "list_target"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        unsafe { for target in &TO_SCAN_QUEUE {
            let log = LogHistory::get_httplog(target.clone());
            let log = match log {
                Some(s) => s,
                None => {
                    continue;
                }
            };

            let request = match log.get_request() {
                Some(r) => r,
                None => {
                    continue;
                }
            };


            println!("{} {}", target, request.get_url());
        } }
        Ok(())
    }

    fn get_detail(&self) -> String {
        format!(r#"list targets that scan queue, when you run command {}, the loaded mod will execute use argument that scan queue
        "#,"scan".green())
    }

    fn get_help(&self) -> String {
        "list_target".to_string()
    }
}

pub struct ActiveScan {
    opts    : CMDOptions
}


impl ActiveScan {
    pub fn new() -> Self {
        Self {
            opts: CMDOptions::default(),
        }
    }
}

impl CMDProc for ActiveScan {
    fn get_name(&self) -> &str {
        "active_scan"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        if line.len() < 3 {
            return Err(STError::new("need a url and name paramters: active_scan ${module} ${url}"));
        }
        println!("[{}] Task running have been move to background, use {} to get more info about task", "*".green() ,"log".green());
        let url = line[2].to_string();
        let modules = get_modules();
        let mut module: Option<&Box<dyn IActive + Sync>> = None;
        for m in modules {
            let meta = match m.metadata() {
                Some(s) => s,
                None => {
                    continue;
                }
            };  

            let name = meta.get_name();
            if name.eq(line[1]) {
                module = Some(m);
            }
        }

        let args = (&line[2..].join(" ")).to_string();
        if module.is_some() {
            std::thread::spawn(move || {
                let module = module.unwrap();
                let meta = module.metadata();
                let meta = match meta {
                    Some(s) => s,
                    None => {
                        return ;
                    }
                };
                let mut running_module = RunningModuleWrapper::new(&meta.get_name(), &args);
                add_running_modules(&mut running_module);
                let start = SystemTime::now();
                let since_the_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                let t1 = since_the_epoch.as_millis();
                let v = module.active_run(&url, HashMap::new());
                let start = SystemTime::now();
                let since_the_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");
                let t2 = since_the_epoch.as_millis();
                let t = t2 - t1;
                info!("{} cost time: {} ms", meta.get_name(), t);
                let state = match &v {
                    Ok(o) => RunningState::DEAD,
                    Err(e) => RunningState::EXCEPTION,
                };
                remove_running_modules(&running_module, t, state);

                match v {
                    Ok(o) => {}
                    Err(e) => {
                        error!("{}", e);
                    }
                };
            });
        }
        
        Ok(())
    }

    fn get_detail(&self) -> String {
        format!(r#"active run module: active_scan ${{module}} ${{url}} ${{opt:options}}
            `{} dir_scan https://127.0.0.1:8080`
        "#,"active_scan".green())
    }

    fn get_help(&self) -> String {
        "active_scan ${module} ${url} ${opt:options}".to_string()
    }
}