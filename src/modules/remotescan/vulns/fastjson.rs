use crate::modules::{IActive, ModuleMeta};


pub struct FastJson {
    meta    : Option<ModuleMeta>
}

impl IActive for FastJson {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>, crate::utils::STError> {
        todo!()
    }

    fn active_run(&self, url: &str, args: crate::modules::Args) -> Result<Vec<crate::modules::Issue>, crate::utils::STError> {
        return Ok(Vec::new())
    }

    fn metadata(&self) -> &Option<crate::modules::ModuleMeta> {
        &self.meta
    }

    fn is_change(&self) -> bool {
        false
    }

    fn update(&mut self) -> Result<(), crate::utils::STError> {
        return Ok(())
    }
}