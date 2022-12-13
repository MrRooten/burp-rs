use crate::{proxy::log::{SiteMap, LogHistory}, st_error, utils::STError};

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
        "list targets that been pushed".to_string()
    }

    fn get_help(&self) -> String {
        "list_target".to_string()
    }
}
