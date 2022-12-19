use crate::{modules::{IActive, ModuleMeta, ModuleType}, proxy::log::LogHistory};



pub struct DirScan {
    meta    : Option<ModuleMeta>
}

impl IActive for DirScan {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>,crate::utils::STError> {
        let result = vec![];
        let log = LogHistory::get_httplog(index);
        return Ok(result);
    }

    fn active_run(&self, url: &str, args: crate::modules::Args) -> Result<Vec<crate::modules::Issue>,crate::utils::STError> {
        let result = Vec::default();
        println!("passive_run...");
        return Ok(result);
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

impl DirScan {
    pub fn new() -> Self {
        let meta = ModuleMeta { name: "dir_scan".to_string(), description: "Scan url".to_string(), m_type: ModuleType::RustModule };
        Self { meta: Some(meta) }
    }
}

