

use log::error;
use rutie::{AnyObject, Object};

use crate::{utils::STError, modules::{IActive, ModuleMeta}, libruby::utils::get_instance};

pub struct RBModule {
    module_script   : String,
    object          : AnyObject
}

impl RBModule {
    pub fn new(file: &str) -> Self {
        let obj = get_instance(file, "RBModule", &[]);
        Self {
            module_script : file.to_string(),
            object: obj,
        }
    }
}


impl IActive for RBModule {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>,STError> {
        todo!()
    }

    fn active_run(&self, url: &str, args: crate::modules::Args) -> Result<Vec<crate::modules::Issue>,STError> {
        todo!()
    }

    fn metadata(&self) -> Option<ModuleMeta> {
        let meta = self.object.protect_send("metadata", &[]);
        let meta = match meta {
            Ok(o) => o,
            Err(e) => {
                error!("{:?}",e);
                return None;
            }
        };

        todo!()
    }
}