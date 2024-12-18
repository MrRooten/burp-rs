use colored::{Colorize, ColoredString};

use crate::{utils::STError, proxy::log::SiteMap, modules::IssueLevel};

use super::cmd_handler::{CMDProc, CMDOptions};

pub struct InfoIssue {
    opts    : CMDOptions
}

impl Default for InfoIssue {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoIssue {
    pub fn new() -> Self {
        Self { opts: Default::default() }
    }
}

impl CMDProc for InfoIssue {
    fn get_name(&self) -> &str {
        "info_issue"
    }

    fn get_opts(&self) -> &CMDOptions {
        &self.opts
    }

    fn process(&self, line: &Vec<&str>) -> Result<(), STError> {
        if line.len() <= 2 {
            return Err(STError::new("Must set args"))
        }
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
                return Err(STError::new("Site does not exist"))
            }
        };

        let issue = site.get_issues();
        let index = line[2].parse::<usize>();
        let index = match index {
            Ok(o) => o,
            Err(e) => {
                return Err(STError::new("Index must be integer"))
            }
        };
        // if issue.len() <= index {
        //     return Err(STError::new("Index out of range"))
        // }


        // println!("Url: {}", issue[index].get_url());
        // println!("Detail: {}",issue[index].get_detail());
        // println!("Name: {}", issue[index].get_name());
        // let log = issue[index].get_httplog();
        // let log = match log {
        //     Some(o) => o,
        //     None => {
        //         return Ok(())
        //     }
        // };

        // let request = log.get_request();
        // let response = log.get_response();
        // println!("Request:\n{}", request.to_string());
        // match response {
        //     Some(o) => {
        //         println!("Response:\n{}",o.to_string())
        //     },
        //     None => {

        //     }
        // };
        Ok(())
    }

    fn get_detail(&self) -> String {
        "Info issue, like: info_issue https://google.com:3".to_string()
    }

    fn get_help(&self) -> String {
        "info_issue ${site:index}".to_string()
    }
}

pub struct ListIssues {
    opts    : CMDOptions
}   

impl Default for ListIssues {
    fn default() -> Self {
        Self::new()
    }
}

impl ListIssues {
    pub fn new() -> Self {
        Self {
            opts    : Default::default()
        }
    }
}

impl CMDProc for ListIssues {
    fn get_name(&self) -> &str {
        "list_issues"
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

        let hosts = map.get_hosts();
        for host in hosts {
            let site = map.get_site(&host).unwrap();
            println!("{}", host.bright_blue());
            let issues = site.get_issues();
            for issue_group in issues {
                let level: ColoredString;
                if !issue_group.1.is_empty() {
                    if issue_group.1.first().unwrap().get_level().eq(&IssueLevel::HighRisk) {
                        level = "high".red();
                    } else if issue_group.1.first().unwrap().get_level().eq(&IssueLevel::Medium) {
                        level = "medium".yellow();
                    } else {
                        level = "low".green();
                    }
                } else {
                    level = "none".green();
                }
                println!("[{}] {}: {} times", level, issue_group.0, issue_group.1.len());
            }
        }
        Ok(())
    }

    fn get_detail(&self) -> String {
        "list the issues that been proof".to_string()
    }

    fn get_help(&self) -> String {
        "list_issues".to_string()
    }
}