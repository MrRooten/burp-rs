use regex::Regex;

use crate::{modules::{IActive, ModuleMeta, ModuleType}, proxy::log::LogHistory, utils::STError, librs::http::utils::{HttpRequest, BParamType}, st_error};

pub struct SSRFDetect {
    meta    : Option<ModuleMeta>
}


impl IActive for SSRFDetect {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>, crate::utils::STError> {
        let url_regex = 
        match Regex::new(r"https?:\/\/(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&\/=]*)") {
            Ok(o) => o,
            Err(e) => {
                return Err(st_error!(e));
            }
        };
        let log = match LogHistory::get_httplog(index) {
            Some(s) => s,
            None => {
                return Err(STError::new("No such a httplog"));
            }
        };

        let request = log.get_request();

        let request = HttpRequest::from_log_request(request);
        let burp = request.to_burp();
        let params = match burp.get_params(){
            Ok(o) => o,
            Err(e) => {
                return Err(e);
            }
        };
        let mut v = vec![];
        for param in &params {
            if param.get_type().eq(&BParamType::GetQuery) || param.get_type().eq(&BParamType::Header) {
                continue;
            }
            let value = param.get_value();
            if url_regex.is_match(value) {
                v.push(param);
            }
        }

        for param in v {

        }
        unimplemented!()
    }

    fn active_run(&self, url: &str, args: crate::modules::Args) -> Result<Vec<crate::modules::Issue>, crate::utils::STError> {
        todo!()
    }

    fn metadata(&self) -> &Option<crate::modules::ModuleMeta> {
        &self.meta
    }

    fn is_change(&self) -> bool {
        false
    }

    fn update(&mut self) -> Result<(), crate::utils::STError> {
        Ok(())
    }
}

impl Default for SSRFDetect {
    fn default() -> Self {
        Self::new()
    }
}

impl SSRFDetect {
    pub fn new() -> Self {
        let meta = ModuleMeta {
            name: "ssrf_detect".to_string(),
            description: "403 bypasser".to_string(),
            m_type: ModuleType::RustModule,
        };
        Self { meta: Some(meta) }
    }
}