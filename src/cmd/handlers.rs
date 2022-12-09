use std::{
    process,
    sync::mpsc::{self, Receiver, Sender},
};

use colored::Colorize;
use hyper::StatusCode;
use log::Level;
use minus::Pager;

use crate::{
    librs::object::object::IObject,
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
        let mut handler = CMDHandler::new();
        handler.init();
        if line.len() == 2 {
            for proc in handler.get_procs() {
                if line[1].eq(proc.get_name()) {
                    println!("{}: {}", proc.get_name().green(), proc.get_help());
                    println!("\t{}", proc.get_detail());
                    return Ok(());
                }
            }
            println!("Command {} not found", line[1]);
        }
        for proc in handler.get_procs() {
            println!("{}: {}", proc.get_name().green(), proc.get_help());
            println!("\t{}", proc.get_detail());
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        "Show help for command".to_string()
    }

    fn get_help(&self) -> String {
        "help ${command}".to_string()
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

    fn get_detail(&self) -> String {
        "Exit program".to_string()
    }

    fn get_help(&self) -> String {
        "exit".to_string()
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

    fn get_detail(&self) -> String {
        return "Show info of proxy".to_string();
    }

    fn get_help(&self) -> String {
        "proxylog_info".to_string()
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
        let p = Pager::new();
        let mut output = String::new();
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

            let c_type = match response {
                Some(r) => match r.get_header("content-type") {
                    Some(v) => v,
                    None => "".to_string(),
                },
                None => "".to_string(),
            };
            let item = format!("{} {} {} {} {}\n", key, url_brief, status, size, c_type);
            output = item + &output;
        }

        match pager(&output, p) {
            Ok(o) => {}
            Err(e) => {
                return Err(st_error!(e));
            }
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        "Show the proxy traffic".to_string()
    }

    fn get_help(&self) -> String {
        "list_history".to_string()
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

    fn get_detail(&self) -> String {
        "Show debug log".to_string()
    }

    fn get_help(&self) -> String {
        "debug_log".to_string()
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
        let index = line[1].to_string().parse::<u32>();
        let index = match index {
            Ok(o) => o,
            Err(e) => {
                return Err(st_error!(e));
            }
        };

        let s = LogHistory::get_httplog(index);
        let s = match s {
            Some(o) => o,
            None => {
                return Err(STError::new("Not such a response"));
            }
        };
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

    fn get_detail(&self) -> String {
        "Show response content".to_string()
    }

    fn get_help(&self) -> String {
        "cat_resp ${{log_id}}".to_string()
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

    fn get_detail(&self) -> String {
        "Clear the screen".to_string()
    }

    fn get_help(&self) -> String {
        "clear".to_string()
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

    fn get_detail(&self) -> String {
        "Setting debug_level by setting level, Showing the debug_level without option".to_string()
    }

    fn get_help(&self) -> String {
        "debug_level ${{opt:level}}".to_string()
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
        let index = line[1].to_string().parse::<u32>();
        let index = match index {
            Ok(o) => o,
            Err(e) => {
                return Err(st_error!(e));
            }
        };

        let s = LogHistory::get_httplog(index);
        let s = match s {
            Some(o) => o,
            None => {
                return Err(STError::new("Not such a request"));
            }
        };
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

    fn get_detail(&self) -> String {
        r#"Show log_id request"#.to_string()
    }

    fn get_help(&self) -> String {
        "cat_req ${{log_id}}".to_string()
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

pub struct GetRequest {
    name: String,
    opts: CMDOptions,
}

impl GetRequest {
    pub fn new() -> Self {
        Self {
            name: "get_req".to_string(),
            opts: Default::default(),
        }
    }
}

impl CMDProc for GetRequest {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &&self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        if line.len() <= 1 {
            let s = format!("{} ${{num}}", self.get_name());
            return Err(STError::new(&s));
        }

        let path = line[1].split(".").collect::<Vec<&str>>();
        let index = path[0].to_string().parse::<u32>();
        let index = match index {
            Ok(o) => o,
            Err(e) => {
                return Err(st_error!(e));
            }
        };
        let mut object_path = String::new();
        if path.len() >= 2 {
            object_path = path[1..path.len()].join(".");
        }

        let s = LogHistory::get_httplog(index);
        let s = match s {
            Some(o) => o,
            None => {
                return Err(STError::new("No such a request"));
            }
        };

        let req = s.get_request();
        let req = match req {
            Some(r) => r,
            None => {
                return Err(STError::new("No such a request in log"));
            }
        };
        println!("{:?}", req.get_object(&object_path));
        Ok(())
    }

    fn get_detail(&self) -> String {
        "Get request info by using path, example ${{logid.url}}".to_string()
    }

    fn get_help(&self) -> String {
        "get_req ${{path}}".to_string()
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

    fn get_detail(&self) -> String {
        "Get debug log information, like take memory size".to_string()
    }

    fn get_help(&self) -> String {
        "log_info".to_string()
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

    fn get_detail(&self) -> String {
        todo!()
    }

    fn get_help(&self) -> String {
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
            let request = LogHistory::get_httplog(*key)
                .unwrap()
                .get_request()
                .unwrap();
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

    fn get_detail(&self) -> String {
        "Get all hosts that traffic are proxied without opt, Get host traffic by using host, Example: sitemap google.com".to_string()
    }

    fn get_help(&self) -> String {
        "sitemap ${{opt:host}}".to_string()
    }
}

impl Sitemap {
    pub fn new() -> Sitemap {
        Sitemap {
            opts: Default::default(),
        }
    }
}

pub static mut TO_SCAN_QUEUE: Vec<u32> = Vec::<u32>::new();
pub static mut SCAN_SENDER: Option<std::sync::mpsc::Sender<u32>> = None::<Sender<u32>>;
pub static mut SCAN_RECEIVER: Option<std::sync::mpsc::Receiver<u32>> = None::<Receiver<u32>>;
pub struct Scan {
    opts: CMDOptions,
}

impl Scan {
    pub fn new() -> Self {
        Self { opts: Default::default() }
    }
}
impl CMDProc for Scan {
    fn get_name(&self) -> &str {
        "scan"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        unsafe {
            if SCAN_SENDER.is_none() {
                let (tx, rx) = mpsc::channel::<u32>();
                SCAN_SENDER = Some(tx);
                SCAN_RECEIVER = Some(rx);
            }
            let sender = match &SCAN_SENDER {
                Some(o) => o,
                None => {
                    return Err(STError::new("SCAN_SENDER is none"));
                }
            };
            while TO_SCAN_QUEUE.len() != 0 {
                let ret = sender.send(TO_SCAN_QUEUE.remove(0));
                match ret {
                    Ok(o) => {},
                    Err(e) => {
                        return Err(st_error!(e))
                    }
                };
            }
            Ok(())
        }
    }

    fn get_detail(&self) -> String {
        "Scan the packet that have been pushed".to_string()
    }

    fn get_help(&self) -> String {
        "scan".to_string()
    }
}

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

        Ok(())
    }

    fn get_detail(&self) -> String {
        "push the history to index".to_string()
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
