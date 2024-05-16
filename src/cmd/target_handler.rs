use std::{
    collections::{HashMap, HashSet}, ptr::addr_of, sync::mpsc, thread, time::{SystemTime, UNIX_EPOCH}
};

use colored::Colorize;
use log::{error, info};
use once_cell::sync::Lazy;

use crate::{
    cmd::handlers::{SCAN_RECEIVER, SCAN_SENDER},
    librs::http::utils::HttpRequest,
    modules::{localscan::PassiveScanner, IActive, ModuleType, Task},
    proxy::log::{LogHistory, LogRequest, LogType, ReqResLog, SiteMap},
    scanner::{
        add_running_modules, get_modules, remove_running_modules, RunningModuleWrapper,
        RunningState,
    },
    st_error,
    utils::STError,
};

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
                let task = Task::new(*log, "dummy", false);
                unsafe {
                    TO_SCAN_QUEUE.push(task);
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
                let task = Task::new(index, "dummy", false);
                TO_SCAN_QUEUE.push(task);
            }
        }

        Ok(())
    }

    fn get_detail(&self) -> String {
        format!(
            r#"push history log to scan queue, like: 
            `{} 1 2 3` 
                push index eq 1 2 3 to scan queue or 
            `{} host:google.com` 
                push host is google indexes to scan queue"#,
            "push".green(),
            "push".green()
        )
    }

    fn get_help(&self) -> String {
        "push ${index}".to_string()
    }
}

impl Default for Push {
    fn default() -> Self {
        Self::new()
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

impl Default for ListTarget {
    fn default() -> Self {
        Self::new()
    }
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
        unsafe {
            for target in &*addr_of!(TO_SCAN_QUEUE) {
                let log = LogHistory::get_httplog(target.get_index());
                let log = match log {
                    Some(s) => s,
                    None => {
                        continue;
                    }
                };

                let request = log.get_request();

                println!("{} {}", target.get_index(), request.get_url());
            }
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        format!(
            r#"list targets that scan queue, when you run command {}, the loaded mod will execute use argument that scan queue
        "#,
            "scan".green()
        )
    }

    fn get_help(&self) -> String {
        "list_target".to_string()
    }
}

pub struct LocalScan {
    opts: CMDOptions,
}

impl Default for LocalScan {
    fn default() -> Self {
        Self::new()
    }
}

impl LocalScan {
    pub fn new() -> Self {
        Self {
            opts: CMDOptions::default(),
        }
    }
}

static mut PASSIVE_SCAN_DONE: Lazy<HashSet<u32>> = Lazy::new(HashSet::new);

impl CMDProc for LocalScan {
    fn get_name(&self) -> &str {
        "local_scan"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        println!("Tasks have been move to background");
        if line.len() >= 2 {
            let history = LogHistory::single();
            let history = match history {
                Some(s) => s,
                None => {
                    return Err(STError::new("Error to get LogHistory"));
                }
            };
            let history = history.get_history();
            let map = match SiteMap::single() {
                Some(s) => s,
                None => {
                    return Err(STError::new("Can not get Sitemap Single instance"));
                }
            };

            let site = map.get_site(line[1]);
            let site = match site {
                Some(s) => s,
                None => {
                    return Err(STError::new("Not exist in logs"));
                }
            };

            let httplog = match map.get_httplogs_by_host(line[1]) {
                Some(log) => log,
                None => {
                    return Err(STError::new("Not exist in logs"));
                }
            };

            thread::spawn(move || {
                let scanner = PassiveScanner::new();
                for target in httplog {
                    unsafe {
                        if PASSIVE_SCAN_DONE.contains(target) {
                            continue;
                        }
                    }
                    scanner.passive_scan(*target);
                    unsafe { PASSIVE_SCAN_DONE.insert(*target) };
                }
            });
        } else {
            let history = LogHistory::single();
            let history = match history {
                Some(s) => s,
                None => {
                    return Err(STError::new("Error to get LogHistory"));
                }
            };
            let history = history.get_history();
            thread::spawn(|| {
                let scanner = PassiveScanner::new();
                for target in history.keys() {
                    unsafe {
                        if PASSIVE_SCAN_DONE.contains(target) {
                            continue;
                        }
                    }
                    scanner.passive_scan(*target);
                    unsafe { PASSIVE_SCAN_DONE.insert(*target) };
                }
            });
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        "Scan http log history, not send request to target".to_string()
    }

    fn get_help(&self) -> String {
        "localscan".to_string()
    }
}

pub struct ActiveScan {
    opts: CMDOptions,
}

impl Default for ActiveScan {
    fn default() -> Self {
        Self::new()
    }
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
            return Err(STError::new(
                "need a url and name paramters: active_scan ${module} ${url}",
            ));
        }
        println!(
            "[{}] Task running have been move to background, use {} to get more info about task",
            "*".green(),
            "log".green()
        );
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

        let args = line[2..].join(" ").to_string();
        if module.is_some() {
            let meta = match module.unwrap().metadata() {
                Some(s) => s,
                None => {
                    return Err(STError::new("error"));
                }
            };

            if meta.get_type().eq(&ModuleType::RubyModule) {
                unsafe {
                    let request = match HttpRequest::from_url(&url) {
                        Ok(o) => o,
                        Err(e) => {
                            return Err(e);
                        }
                    };
                    let log_request = LogRequest::from_http_request(&request);
                    let mut reqreslog = ReqResLog::new(log_request);
                    reqreslog.set_type(LogType::TempForActive);
                    let httplog = match LogHistory::single() {
                        Some(s) => s,
                        None => {
                            return Err(STError::new("error to get history log"));
                        }
                    };
                    let scan_index = match httplog.push_log(reqreslog) {
                        Ok(o) => o,
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    if SCAN_SENDER.is_none() {
                        let (tx, rx) = mpsc::channel::<Task>();
                        SCAN_SENDER = Some(tx);
                        SCAN_RECEIVER = Some(rx);
                    }
                    let sender = match &*addr_of!(SCAN_SENDER) {
                        Some(o) => o,
                        None => {
                            return Err(STError::new("SCAN_SENDER is none"));
                        }
                    };
                    let task = Task::new(scan_index, line[1], true);
                    let ret = sender.send(task);
                    match ret {
                        Ok(o) => {}
                        Err(e) => return Err(st_error!(e)),
                    };
                }
            } else {
                std::thread::spawn(move || {
                    let module = module.unwrap();
                    let meta = module.metadata();
                    let meta = match meta {
                        Some(s) => s,
                        None => {
                            return;
                        }
                    };
                    let mut running_module = RunningModuleWrapper::new(meta.get_name(), &args);
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
        }

        Ok(())
    }

    fn get_detail(&self) -> String {
        format!(
            r#"active run module: active_scan ${{module}} ${{url}} ${{opt:options}}
            `{} dir_scan https://127.0.0.1:8080`
        "#,
            "active_scan".green()
        )
    }

    fn get_help(&self) -> String {
        "active_scan ${module} ${url} ${opt:options}".to_string()
    }
}
