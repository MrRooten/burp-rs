use std::{
    process, ptr::addr_of, sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    }
};

use colored::{ColoredString, Colorize};
use hyper::StatusCode;
use log::Level;
use minus::Pager;

use crate::{
    librs::object::object_inner::IObject,
    modules::Task,
    proxy::log::{LogHistory, ReqResLog, SiteMap},
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
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
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
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
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
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
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
        "Show info of proxy".to_string()
    }

    fn get_help(&self) -> String {
        "proxylog_info".to_string()
    }
}

impl Default for ProxyLogInfo {
    fn default() -> Self {
        Self::new()
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

impl Default for ListHistory {
    fn default() -> Self {
        Self::new()
    }
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

            let p = Pager::new();
            let mut output = String::new();
            for key in httplog {
                let request = history.get(key).unwrap().get_request();
                let response = history.get(key).unwrap().get_response();
                let status = match &*response.borrow() {
                    Some(r) => r.get_status(),
                    None => StatusCode::GONE,
                };
                let size = match &*response.borrow() {
                    Some(r) => r.get_size(),
                    None => 0,
                };

                let mut url_brief = request.get_url();
                if url_brief.len() > 61 {
                    url_brief = url_brief[0..60].to_string();
                    url_brief.push_str("...");
                }

                let c_type = match &*response.borrow() {
                    Some(r) => match r.get_header("content-type") {
                        Some(v) => v,
                        None => "".to_string(),
                    },
                    None => "".to_string(),
                };
                let status_s: ColoredString;
                let status_first = status.as_u16() / 100;
                if status_first == 1 {
                    status_s = status.as_str().green();
                } else if status_first == 2 {
                    status_s = status.as_str().bright_blue();
                } else if status_first == 3 {
                    status_s = status.as_str().yellow();
                } else if status_first == 4 {
                    status_s = status.as_str().red();
                } else {
                    status_s = status.as_str().bright_red();
                }
                let item = format!(
                    "{} {} {} {} {}\n",
                    key,
                    url_brief.yellow(),
                    status_s,
                    size,
                    c_type.bright_blue()
                );
                output = item + &output;
            }

            if output.split('\n').count() < 50 {
                println!("{}", output);
            } else {
                let p = Pager::new();
                match pager(&output, p) {
                    Ok(o) => {}
                    Err(e) => {
                        return Err(st_error!(e));
                    }
                }
            }
            Ok(())
        } else {
            let history = LogHistory::single();
            let history = match history {
                Some(s) => s,
                None => {
                    return Err(STError::new("Error to get LogHistory"));
                }
            };

            let history = history.get_history();
            let mut keys = history.keys().collect::<Vec<&u32>>();
            if keys.is_empty() {
                return Err(STError::new("No log"));
            }
            keys.sort();
            let p = Pager::new();
            let mut output = String::new();
            for key in keys {
                let request = history.get(key).unwrap().get_request();
                let response = history.get(key).unwrap().get_response();
                let status = match &*response.borrow() {
                    Some(r) => r.get_status(),
                    None => StatusCode::GONE,
                };
                let size = match &*response.borrow() {
                    Some(r) => r.get_size(),
                    None => 0,
                };

                let mut url_brief = request.get_url();
                if url_brief.len() > 61 {
                    url_brief = url_brief[0..60].to_string();
                    url_brief.push_str("...");
                }

                let c_type = match &*response.borrow() {
                    Some(r) => match r.get_header("content-type") {
                        Some(v) => v,
                        None => "".to_string(),
                    },
                    None => "".to_string(),
                };
                let status_s: ColoredString;
                let status_first = status.as_u16() / 100;
                if status_first == 1 {
                    status_s = status.as_str().green();
                } else if status_first == 2 {
                    status_s = status.as_str().bright_blue();
                } else if status_first == 3 {
                    status_s = status.as_str().yellow();
                } else if status_first == 4 {
                    status_s = status.as_str().red();
                } else {
                    status_s = status.as_str().bright_red();
                }
                let item = format!(
                    "{} {} {} {} {}\n",
                    key,
                    url_brief.yellow(),
                    status_s,
                    size,
                    c_type.bright_blue()
                );
                output = item + &output;
            }

            if output.split('\n').count() < 50 {
                println!("{}", output);
            } else {
                let p = Pager::new();
                match pager(&output, p) {
                    Ok(o) => {}
                    Err(e) => {
                        return Err(st_error!(e));
                    }
                }
            }
            Ok(())
        }
    }

    fn get_detail(&self) -> String {
        "Show the proxy traffic".to_string()
    }

    fn get_help(&self) -> String {
        "list_history".to_string()
    }
}

pub struct Log {
    name: String,
    opts: CMDOptions,
}

impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}

impl Log {
    pub fn new() -> Self {
        Self {
            name: "log".to_string(),
            opts: Default::default(),
        }
    }
}

impl CMDProc for Log {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        unsafe {
            for log in &*addr_of!(LOGS) {
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
        let s = match &*s.get_response().borrow() {
            Some(s) => s.get_beauty_string(),
            None => "".to_string(),
        };
        if s.split('\n').count() < 50 {
            println!("{}", s);
        } else {
            let p = Pager::new();
            match pager(&s, p) {
                Ok(o) => {}
                Err(e) => {
                    return Err(st_error!(e));
                }
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

impl Default for CatResponse {
    fn default() -> Self {
        Self::new()
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

impl Default for ClearScreen {
    fn default() -> Self {
        Self::new()
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
                println!("{}", LEVEL);
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
        Ok(())
    }

    fn get_detail(&self) -> String {
        "Setting debug_level by setting level, Showing the debug_level without option".to_string()
    }

    fn get_help(&self) -> String {
        "debug_level ${{opt:level}}".to_string()
    }
}

impl Default for DebugLevel {
    fn default() -> Self {
        Self::new()
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
        let s = s.get_request().as_string();
        let p = Pager::new();
        if s.split('\n').count() < 50 {
            println!("{}", s);
        } else {
            let p = Pager::new();
            match pager(&s, p) {
                Ok(o) => {}
                Err(e) => {
                    return Err(st_error!(e));
                }
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

impl Default for CatRequest {
    fn default() -> Self {
        Self::new()
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

impl Default for GetRequest {
    fn default() -> Self {
        Self::new()
    }
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
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        if line.len() <= 1 {
            let s = format!("{} ${{num}}", self.get_name());
            return Err(STError::new(&s));
        }

        let path = line[1].split('.').collect::<Vec<&str>>();
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
        let output = match req.get_object(&object_path) {
            Some(s) => s,
            None => "\"\"".to_string(),
        };
        println!("{}: {}", object_path, output);
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

impl Default for DebugLogInfo {
    fn default() -> Self {
        Self::new()
    }
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
            for log in &*addr_of!(LOGS) {
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
        "search_log"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        if line.len() < 2 {
            return Err(STError::new("Need search string: search_log ${target}"));
        }
        let target = line[1];
        let history = LogHistory::single();
        let history = match history {
            Some(s) => s,
            None => {
                return Err(STError::new("Error to get LogHistory"));
            }
        };

        let history = history.get_history();
        let mut keys = history.keys().collect::<Vec<&u32>>();
        if keys.is_empty() {
            return Err(STError::new("No log"));
        }
        keys.sort();
        let p = Pager::new();
        let mut output = String::new();
        for key in keys {
            let request = history.get(key).unwrap().get_request();
            let response = history.get(key).unwrap().get_response();
            let mut flag = false;
            if let Some(r) = &*response.borrow() {
                if request.contains(target, false) || r.contains(target, false) {
                    flag = true;
                }
            } else if request.contains(target, false) {
                flag = true;
            }

            if !flag {
                continue;
            }
            let status = match &*response.borrow() {
                Some(r) => r.get_status(),
                None => StatusCode::GONE,
            };
            let size = match &*response.borrow() {
                Some(r) => r.get_size(),
                None => 0,
            };

            let mut url_brief = request.get_url();
            if url_brief.len() > 61 {
                url_brief = url_brief[0..60].to_string();
                url_brief.push_str("...");
            }

            let c_type = match &*response.borrow() {
                Some(r) => match r.get_header("content-type") {
                    Some(v) => v,
                    None => "".to_string(),
                },
                None => "".to_string(),
            };
            let status_s: ColoredString;
            let status_first = status.as_u16() / 100;
            if status_first == 1 {
                status_s = status.as_str().green();
            } else if status_first == 2 {
                status_s = status.as_str().bright_blue();
            } else if status_first == 3 {
                status_s = status.as_str().yellow();
            } else if status_first == 4 {
                status_s = status.as_str().red();
            } else {
                status_s = status.as_str().bright_red();
            }
            let item = format!(
                "{} {} {} {} {}\n",
                key,
                url_brief.yellow(),
                status_s,
                size,
                c_type.bright_blue()
            );
            output = item + &output;
        }

        if output.split('\n').count() < 50 {
            println!("{}", output);
        } else {
            let p = Pager::new();
            match pager(&output, p) {
                Ok(o) => {}
                Err(e) => {
                    return Err(st_error!(e));
                }
            }
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        "filter the proxy log".to_string()
    }

    fn get_help(&self) -> String {
        "search_log ${string}".to_string()
    }
}

impl Default for SearchLog {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchLog {
    pub fn new() -> Self {
        SearchLog {
            opts: Default::default(),
        }
    }
}

fn size_to_human_readable(size: usize) -> String {
    let mut ret = String::new();
    if size < 1024 {
        ret = format!("{} byte", size);
    } else if size > 1024 && size < 1024 * 1024 {
        ret = format!("{} KB", size / 1024);
    } else if size > 1024 * 1024 {
        let s = (size / (1024 * 1024)) as f32;
        ret = format!("{} MB", s);
    }
    ret
}
pub struct Sitemap {
    opts: CMDOptions,
}

impl CMDProc for Sitemap {
    fn get_name(&self) -> &str {
        "sitemap"
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
            if result.split('\n').count() < 50 {
                println!("{}", result);
            } else {
                let p = Pager::new();
                match pager(&result, p) {
                    Ok(o) => {}
                    Err(e) => {
                        return Err(st_error!(e));
                    }
                }
            }

            return Ok(());
        }
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
        let mut request_num = 0;
        let mut response_num = 0;
        let mut request_size = 0;
        let mut response_size = 0;
        for key in httplog {
            let request = LogHistory::get_httplog(*key).unwrap().get_request();
            let response = LogHistory::get_httplog(*key).unwrap().get_response();
            let status = match &*response.borrow() {
                Some(r) => {
                    response_size += r.get_body().len();
                    r.get_status()
                }
                None => StatusCode::GONE,
            };
            let size = match &*response.borrow() {
                Some(r) => r.get_size(),
                None => 0,
            };

            let mut url_brief = request.get_url();
            if url_brief.len() > 61 {
                url_brief = url_brief[0..60].to_string();
                url_brief.push_str("...");
            }
            request_num += 1;
            response_num += 1;
            request_size += request.get_body().len();
        }

        println!("Request num:{}", request_num);
        println!("Response num:{}", response_num);
        println!("Request size:{}", size_to_human_readable(request_size));
        println!("Response size:{}", size_to_human_readable(response_size));
        let paths = site.get_paths();
        for path in paths {
            println!("  {}", path);
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

impl Default for Sitemap {
    fn default() -> Self {
        Self::new()
    }
}

impl Sitemap {
    pub fn new() -> Sitemap {
        Sitemap {
            opts: Default::default(),
        }
    }
}

pub static mut TO_SCAN_QUEUE: Vec<Task> = Vec::<Task>::new();
pub static mut SCAN_SENDER: Option<std::sync::mpsc::Sender<Task>> = None::<Sender<Task>>;
pub static mut SCAN_RECEIVER: Option<std::sync::mpsc::Receiver<Task>> = None::<Receiver<Task>>;
pub struct Scan {
    opts: CMDOptions,
}

impl Default for Scan {
    fn default() -> Self {
        Self::new()
    }
}

impl Scan {
    pub fn new() -> Self {
        Self {
            opts: Default::default(),
        }
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
            while !TO_SCAN_QUEUE.is_empty() {
                let ret = sender.send(TO_SCAN_QUEUE.remove(0));
                match ret {
                    Ok(o) => {}
                    Err(e) => return Err(st_error!(e)),
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

pub struct Test {
    opts: CMDOptions,
}

impl Default for Test {
    fn default() -> Self {
        Self::new()
    }
}

impl Test {
    pub fn new() -> Self {
        Self {
            opts: Default::default(),
        }
    }
}

impl CMDProc for Test {
    fn get_name(&self) -> &str {
        "test"
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

        let request = s.get_request();
        println!("{:?}", request.get_params());
        Ok(())
    }

    fn get_detail(&self) -> String {
        "test some functions".to_string()
    }

    fn get_help(&self) -> String {
        "test".to_string()
    }
}

pub struct Filter {
    opts: CMDOptions,
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

impl Filter {
    pub fn new() -> Self {
        Self {
            opts: Default::default(),
        }
    }
}

impl CMDProc for Filter {
    fn get_name(&self) -> &str {
        "filter"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        #[derive(PartialEq)]
        enum MatchFlag {
            Contains,
            Equal,
            Exist,
        }
        let flag: MatchFlag;
        let kv: Vec<&str>;
        let mut httplog: Vec<(&u32, &Arc<ReqResLog>)> = vec![];
        if line[1].split("==").collect::<Vec<&str>>().len() == 2 {
            flag = MatchFlag::Equal;
            kv = line[1].split("==").collect::<Vec<&str>>();
        } else if line[1].split('=').collect::<Vec<&str>>().len() == 2 {
            flag = MatchFlag::Contains;
            kv = line[1].split('=').collect::<Vec<&str>>();
        } else {
            flag = MatchFlag::Exist;
            kv = line[1].split("==").collect::<Vec<&str>>();
        }

        let path = kv[0].split('.').collect::<Vec<&str>>();

        if path[0].eq("req") {
            let mut object_path = String::new();
            if path.len() >= 2 {
                object_path = path[1..path.len()].join(".");
            }

            let history = LogHistory::single();
            let history = match history {
                Some(s) => s,
                None => {
                    return Err(STError::new("Error to get LogHistory"));
                }
            };
            let history = history.get_history();
            for h in history {
                let req = h.1.get_request();
                let output = match req.get_object(&object_path) {
                    Some(s) => s,
                    None => "\"\"".to_string(),
                };

                if flag.eq(&MatchFlag::Contains) {
                    let value = kv[1];
                    if output.contains(value) {
                        httplog.push(h);
                    }
                } else if flag.eq(&MatchFlag::Equal) {
                    let value = kv[1];
                    if output.eq(value) {
                        httplog.push(h);
                    }
                } else if flag.eq(&MatchFlag::Exist) {
                    httplog.push(h);
                }
            }
        } else if path[0].eq("resp") {
            let mut object_path = String::new();
            if path.len() >= 2 {
                object_path = path[1..path.len()].join(".");
            }

            let history = LogHistory::single();
            let history = match history {
                Some(s) => s,
                None => {
                    return Err(STError::new("Error to get LogHistory"));
                }
            };
            let history = history.get_history();
            for h in history {
                let resp = h.1.get_response();
                let v = &*resp.borrow();
                let resp = match v {
                    Some(s) => s,
                    None => {
                        continue;
                    }
                };
                let output = match resp.get_object(&object_path) {
                    Some(s) => s,
                    None => "\"\"".to_string(),
                };

                if flag.eq(&MatchFlag::Contains) {
                    let value = kv[1];
                    if output.contains(value) {
                        httplog.push(h);
                    }
                } else if flag.eq(&MatchFlag::Equal) {
                    let value = kv[1];
                    if output.eq(value) {
                        httplog.push(h);
                    }
                } else if flag.eq(&MatchFlag::Exist) {
                    httplog.push(h);
                }
            }
        }
        let mut output = String::new();
        for key in httplog {
            let request = key.1.get_request();
            let response = key.1.get_response();
            let status = match &*response.borrow() {
                Some(r) => r.get_status(),
                None => StatusCode::GONE,
            };
            let size = match &*response.borrow() {
                Some(r) => r.get_size(),
                None => 0,
            };

            let mut url_brief = request.get_url();
            if url_brief.len() > 61 {
                url_brief = url_brief[0..60].to_string();
                url_brief.push_str("...");
            }

            let c_type = match &*response.borrow() {
                Some(r) => match r.get_header("content-type") {
                    Some(v) => v,
                    None => "".to_string(),
                },
                None => "".to_string(),
            };
            let status_s: ColoredString;
            let status_first = status.as_u16() / 100;
            if status_first == 1 {
                status_s = status.as_str().green();
            } else if status_first == 2 {
                status_s = status.as_str().bright_blue();
            } else if status_first == 3 {
                status_s = status.as_str().yellow();
            } else if status_first == 4 {
                status_s = status.as_str().red();
            } else {
                status_s = status.as_str().bright_red();
            }
            let item = format!(
                "{} {} {} {} {}\n",
                key.0,
                url_brief.yellow(),
                status_s,
                size,
                c_type.bright_blue()
            );
            output = item + &output;
        }

        if output.split('\n').count() < 50 {
            println!("{}", output);
        } else {
            let p = Pager::new();
            match pager(&output, p) {
                Ok(o) => {}
                Err(e) => {
                    return Err(st_error!(e));
                }
            }
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        "filter log output".to_string()
    }

    fn get_help(&self) -> String {
        "filter ${req.url=bing.com}".to_string()
    }
}
