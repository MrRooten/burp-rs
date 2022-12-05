use std::process;

use colored::Colorize;
use hyper::StatusCode;
use log::Level;
use minus::Pager;

use crate::{
    proxy::log::{LogHistory, SiteMap},
    st_error,
    utils::{
        log::{LEVEL, LOGS},
        STError,
    },
};

use super::{cmd_handler::*, pager::pager};

#[derive(Default)]
pub struct Helper {
    name: String,
    opts: CMDOptions,
}

impl CMDProc for Helper {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_opts(&self) -> &CMDOptions {
        return &self.opts;
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), crate::utils::STError> {
        println!("help");
        Ok(())
    }
}

impl Helper {
    pub fn new() -> Self {
        Self {
            name: "help".to_string(),
            opts: Default::default(),
        }
    }
}
#[derive(Default)]
pub struct Exit {
    name: String,
    opts: CMDOptions,
}

impl CMDProc for Exit {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_opts(&self) -> &CMDOptions {
        return &self.opts;
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), crate::utils::STError> {
        process::exit(0);
    }
}

impl Exit {
    pub fn new() -> Self {
        Self {
            name: "exit".to_string(),
            opts: Default::default(),
        }
    }
}

pub struct ProxyLogInfo {
    name: String,
    opts: CMDOptions,
}

impl CMDProc for ProxyLogInfo {
    fn get_name(&self) -> &str {
        return &self.name;
    }

    fn get_opts(&self) -> &CMDOptions {
        return &self.opts;
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), crate::utils::STError> {
        let history = LogHistory::single();
        let history = match history {
            Some(s) => s,
            None => {
                return Err(STError::new("Error to get LogHistory"));
            }
        };
        let size = history.get_size();
        if size < 1024 {
            println!("Proxy traffic size: {} byte", size);
        } else if size > 1024 && size < 1024 * 1024 {
            println!("Proxy traffic size: {} KB", size / 1024);
        } else if size > 1024 * 1024 {
            println!("Proxy traffic size: {} MB", size / (1024 * 1024));
        }
        println!("Proxy Request:{}", history.get_req_num());
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
    name: String,
    opts: CMDOptions,
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

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
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
                None => StatusCode::GONE,
            };
            let size = match response {
                Some(r) => r.get_size(),
                None => 0,
            };

            let mut url_brief = request.get_url();
            if url_brief.len() > 61 {
                url_brief = url_brief[0..60].to_string();
                url_brief.push_str("...");
            }

            println!("{} {} {} {}", key, url_brief, status, size);
        }
        Ok(())
    }
}

pub struct DebugLog {
    name: String,
    opts: CMDOptions,
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
            for log in &LOGS {
                println!("{}", log);
            }
        }

        Ok(())
    }
}

pub struct CatResponse {
    name: String,
    opts: CMDOptions,
}

impl CMDProc for CatResponse {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        if line.len() <= 1 {
            let s = format!("{} ${{num}}", self.get_name());
            return Err(STError::new(&s));
        }
        let index = line[1].to_string().parse::<u32>().unwrap();

        let s = LogHistory::get_httplog(index).unwrap();
        let s = match s.get_response() {
            Some(s) => s.get_beauty_string(),
            None => "".to_string(),
        };
        let p = Pager::new();
        match pager(&s, p) {
            Ok(o) => {}
            Err(e) => {
                return Err(st_error!(e));
            }
        }
        Ok(())
    }
}

impl CatResponse {
    pub fn new() -> Self {
        Self {
            name: "cat_resp".to_string(),
            opts: Default::default(),
        }
    }
}

pub struct ClearScreen {
    name: String,
    opts: CMDOptions,
}

impl CMDProc for ClearScreen {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        Ok(())
    }
}

impl ClearScreen {
    pub fn new() -> Self {
        Self {
            name: "clear".to_string(),
            opts: Default::default(),
        }
    }
}

pub struct DebugLevel {
    name: String,
    opts: CMDOptions,
}

impl CMDProc for DebugLevel {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        unsafe {
            if line.len() == 1 {
                println!("{}", LEVEL.to_string());
                return Ok(());
            }
            if line[1].eq("info") {
                LEVEL = Level::Info;
            } else if line[1].eq("debug") {
                LEVEL = Level::Debug;
            } else if line[1].eq("warn") {
                LEVEL = Level::Warn;
            } else if line[1].eq("error") {
                LEVEL = Level::Error;
            } else {
                println!("Only support <info> <debug> <warn> <error>");
            }
        }
        return Ok(());
    }
}

impl DebugLevel {
    pub fn new() -> Self {
        Self {
            name: "debug_level".to_string(),
            opts: Default::default(),
        }
    }
}

pub struct CatRequest {
    name: String,
    opts: CMDOptions,
}

impl CMDProc for CatRequest {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        if line.len() <= 1 {
            let s = format!("{} ${{num}}", self.get_name());
            return Err(STError::new(&s));
        }
        let index = line[1].to_string().parse::<u32>().unwrap();

        let s = LogHistory::get_httplog(index).unwrap();
        let s = s.get_request().unwrap().to_string();
        let p = Pager::new();
        match pager(&s, p) {
            Ok(o) => {}
            Err(e) => {
                return Err(st_error!(e));
            }
        }
        Ok(())
    }
}

impl CatRequest {
    pub fn new() -> Self {
        Self {
            name: "cat_req".to_string(),
            opts: Default::default(),
        }
    }
}

pub struct DebugLogInfo {
    name: String,
    opts: CMDOptions,
}

impl DebugLogInfo {
    pub fn new() -> Self {
        Self {
            name: "log_info".to_string(),
            opts: Default::default(),
        }
    }
}

impl CMDProc for DebugLogInfo {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        unsafe {
            let mut size = 0;
            let mut num = 0;
            for log in &LOGS {
                size += log.as_bytes().len();
                num += 1;
            }
            if size < 1024 {
                println!("Log size: {} byte", size);
            } else if size > 1024 && size < 1024 * 1024 {
                println!("Log size: {} KB", size / 1024);
            } else if size > 1024 * 1024 {
                let s = (size / (1024 * 1024)) as f32;
                println!("Log size: {} MB", s);
            }
            println!("Log num: {}", num);
        }

        Ok(())
    }
}

pub struct SearchLog {
    opts: CMDOptions,
}

impl CMDProc for SearchLog {
    fn get_name(&self) -> &str {
        return "search_log";
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        todo!()
    }
}

impl SearchLog {
    pub fn new() -> Self {
        SearchLog {
            opts: Default::default(),
        }
    }
}

pub struct Sitemap {
    opts: CMDOptions,
}

impl CMDProc for Sitemap {
    fn get_name(&self) -> &str {
        return "sitemap";
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        let map = match SiteMap::single() {
            Some(s) => s,
            None => {
                return Err(STError::new("Can not get Sitemap Single instance"));
            }
        };
        if line.len() <= 1 {
            let hosts = map.get_hosts();
            let mut result = String::new();
            for host in hosts {
                let push = format!("host: {}\n", host.green());
                result.push_str(&push);
            }
            let p = Pager::new();
            match pager(&result, p) {
                Ok(o) => {}
                Err(e) => {
                    return Err(st_error!(e));
                }
            }

            return Ok(());
        }

        let httplog = match map.get_httplogs_by_host(line[1]) {
            Some(log) => log,
            None => {
                return Err(STError::new("Not exist in logs"));
            }
        };

        for key in httplog {
            let request = LogHistory::get_httplog(*key).unwrap().get_request().unwrap();
            let response = LogHistory::get_httplog(*key).unwrap().get_response();
            let status = match response {
                Some(r) => r.get_status(),
                None => StatusCode::GONE,
            };
            let size = match response {
                Some(r) => r.get_size(),
                None => 0,
            };

            let mut url_brief = request.get_url();
            if url_brief.len() > 61 {
                url_brief = url_brief[0..60].to_string();
                url_brief.push_str("...");
            }

            println!("{} {} {} {}", key, url_brief, status, size);
        }
        Ok(())
    }
}

impl Sitemap {
    pub fn new() -> Sitemap {
        Sitemap {
            opts: Default::default(),
        }
    }
}
