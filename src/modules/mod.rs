pub mod remotescan;
pub mod localscan;
pub mod lib;

#[derive(Default)]
pub struct Helper {}

#[derive(PartialEq)]
pub enum IssueLevel {
    Info,
    Medium,
    HighRisk,
    Critical,
}

pub struct Issue {
    name: String,
    detail: String,
    level: IssueLevel,
    confidence: IssueConfidence,
    httplog: Option<Arc<ReqResLog>>,
    host: String,
}

pub enum IssueConfidence {
    Suspicious,
    Confirm,
}

impl Issue {
    pub fn new(
        name: &str,
        level: IssueLevel,
        detail: &str,
        confidance: IssueConfidence,
        host: &str,
    ) -> Issue {
        Self {
            name: name.to_string(),
            detail: detail.to_string(),
            level,
            confidence: confidance,
            httplog: None,
            host: host.to_string(),
        }
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    /** `add_issue` Push issue to Site of Sitemap index by hostname


    ```
    Issue::add_issue(issue, httplog);
    ```
    */
    pub fn add_issue(mut issue: Issue, httplog: &Arc<ReqResLog>) {
        issue.set_httplog(httplog);
        let sitemap = SiteMap::single();
        let sitemap = match sitemap {
            Some(s) => s,
            None => {
                return;
            }
        };
        let issue = Arc::new(issue);
        match sitemap.push_issue(issue) {
            Ok(()) => {

            },
            Err(e) => {
                error!("{}",e);
            }
        }
    }

    pub fn get_issues() -> Vec<&'static Arc<Issue>> {
        let mut ret = Vec::default();
        let sitemap = SiteMap::single();
        let sitemap = match sitemap {
            Some(s) => s,
            None => {
                return Default::default();
            }
        };

        let hosts = sitemap.get_hosts();
        for host in hosts {
            let site = sitemap.get_site(&host);
            let site = match site {
                Some(s) => s,
                None => {
                    continue;
                }
            };

            let issues = site.get_issues();
            for issues_group in issues.values() {
                for issue in issues_group {
                    ret.push(issue);
                }
            }
        }
        ret
    }

    pub fn set_httplog(&mut self, httplog: &Arc<ReqResLog>) {
        self.httplog = Some(Arc::clone(httplog));
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_detail(&self) -> &str {
        &self.detail
    }

    pub fn get_level(&self) -> &IssueLevel {
        &self.level
    }

    fn get_confidence(&self) -> &IssueConfidence {
        &self.confidence
    }

    pub fn get_httplog(&self) -> &Option<Arc<ReqResLog>> {
        &self.httplog
    }

    pub fn get_url(&self) -> String {
        let httplog = match &self.httplog {
            Some(s) => s,
            None => {
                return "".to_string();
            }
        };

        let request = httplog.get_request();

        request.get_url()
    }


}

#[allow(clippy::ptr_arg)]
pub trait IPassive {
    fn run(&self, log: &Arc<ReqResLog>, burp: &BurpRequest, params: &Vec<BurpParam>) -> Result<(), STError>;

    fn name(&self) -> String;

    fn help(&self) -> Helper;
}

use std::{collections::{HashMap, HashSet}, ptr::{addr_of, addr_of_mut}, sync::Arc};

use log::error;
use once_cell::unsync::Lazy;
use wildmatch::WildMatch;

use crate::{
    cmd::handlers::SCAN_RECEIVER,
    proxy::log::{ReqResLog, SiteMap},
    utils::STError, librs::http::utils::{BurpRequest, BurpParam},
};

#[derive(Debug,Clone, PartialEq,Hash)]
pub enum ModuleType {
    RubyModule,
    RustModule
}
#[derive(Debug,Clone,PartialEq,Hash)]
pub struct ModuleMeta {
    name: String,
    description: String,
    m_type      : ModuleType
}

impl Eq for ModuleMeta {

}


impl ModuleMeta {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_type(&self) -> &ModuleType {
        &self.m_type
    }

}
type Args = HashMap<String, String>;

pub trait IActive {
    //Use in proxy mod
    fn passive_run(&self, index: u32) -> Result<Vec<Issue>, STError>;
    //Use in cmd mod
    fn active_run(&self, url: &str, args: Args) -> Result<Vec<Issue>, STError>;

    fn metadata(&self) -> &Option<ModuleMeta>;

    fn is_change(&self) -> bool;

    fn update(&mut self) -> Result<(), STError>;
}

pub static mut GLOB_MODS: Vec<ModuleMeta> = Vec::<ModuleMeta>::new();
pub static mut WILL_RUN_MODS:Lazy<HashSet<ModuleMeta>> = Lazy::new(|| {HashSet::<ModuleMeta>::new()});
pub fn get_modules_meta() -> &'static Vec<ModuleMeta> {
    unsafe {
        &*addr_of!(GLOB_MODS)
    }
}

pub fn get_will_run_mods() -> &'static HashSet<ModuleMeta> {
    unsafe {
        &*addr_of!(WILL_RUN_MODS)
    }
}

/** `push_will_run_mod` Push poc you want to run, Support wildcard match


```
push_will_run_mod("crlf*")
```
*/
pub fn push_will_run_mod(name: &str) {
    unsafe {
        for poc in get_modules_meta() {
            if WildMatch::new(name).matches(poc.get_name()) {
                WILL_RUN_MODS.insert(poc.clone());
            }
        }
    }
}

pub fn remove_loaded_mod(name: &str) {
    unsafe {
        for poc in get_modules_meta() {
            if WildMatch::new(name).matches(poc.get_name()) {
                WILL_RUN_MODS.remove(poc);
            }
        }
    }
}

pub fn remove_will_run_mod(name: &str) {
    unsafe {
        WILL_RUN_MODS.retain(|x| !WildMatch::new(name).matches(x.get_name()));
    }
}

pub struct Task {
    once    : bool,
    index   : u32,
    mod_name: String
}

impl Task {
    pub fn new(index: u32, mod_name: &str, once: bool) -> Self {
        Self {
            index,
            once,
            mod_name : mod_name.to_string()
        }
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn is_once(&self) -> bool {
        self.once
    }

    pub fn get_mod_name(&self) -> &String {
        &self.mod_name
    }
}

pub fn get_next_to_scan() -> Option<Task> {
    unsafe {
        let receiver = &mut *addr_of_mut!(SCAN_RECEIVER);
        let receiver = match receiver {
            Some(o) => o,
            None => {
                //Httplog does not have 0 index
                panic!("Receiver")
            }
        };

        match receiver.try_recv() {
            Ok(o) => Some(o),
            Err(e) => None
        }
    }
}

