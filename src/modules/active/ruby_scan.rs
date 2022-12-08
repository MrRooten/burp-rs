use std::fs;

use log::error;
use rutie::{eval, AnyObject, Object};

use crate::{
    libruby::utils::{call_object_method, get_instance, object_to_string},
    modules::{IActive, ModuleMeta},
    utils::STError, st_error,
};

pub struct RBModule {
    module_script: String,
    object: AnyObject,
}

impl RBModule {
    pub fn new(file: &str) -> Result<Self, STError> {
        let f = fs::read_to_string(file);
        let s = match f {
            Ok(o) => o,
            Err(e) => {
                return Err(st_error!(e));
            }
        };
        let _ = eval!(&s);
        let obj = get_instance(file, "RBModule", &[]);
        Ok(Self {
            module_script: file.to_string(),
            object: obj,
        })
    }
}

impl IActive for RBModule {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>, STError> {
        todo!()
    }

    fn active_run(
        &self,
        url: &str,
        args: crate::modules::Args,
    ) -> Result<Vec<crate::modules::Issue>, STError> {
        todo!()
    }

    fn metadata(&self) -> Option<ModuleMeta> {
        let meta = self.object.protect_send("metadata", &[]);
        let meta = match meta {
            Ok(o) => o,
            Err(e) => {
                error!("{:?}", e);
                return None;
            }
        };
        let meta = call_object_method(&self.object, "metadata", &[]);
        let meta = match meta {
            Ok(o) => o,
            Err(e) => return None,
        };

        let meta_json = match object_to_string(&meta) {
            Ok(o) => o,
            Err(e) => return None,
        };

        println!("{}", meta_json);
        unimplemented!()
    }
}
