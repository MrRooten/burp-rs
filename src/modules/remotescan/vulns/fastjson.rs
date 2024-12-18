use crate::{modules::{IActive, ModuleMeta}, proxy::log::LogHistory, utils::STError};


pub struct FastJson {
    meta    : Option<ModuleMeta>
}

impl IActive for FastJson {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>, crate::utils::STError> {
        let log = match LogHistory::get_httplog(index) {
            Some(s) => s,
            None => {
                let s = format!("No such a index: {}", index);
                return Err(STError::new(&s));
            }
        };

        let request = log.get_request();
        if let Some(ctype) = request.get_header("content-type") {
            if ctype.contains("json") {
                
            }
        }
        let params = log.get_request().get_params();
        todo!()
    }

    fn active_run(&self, url: &str, args: crate::modules::Args) -> Result<Vec<crate::modules::Issue>, crate::utils::STError> {
        Ok(Vec::new())
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