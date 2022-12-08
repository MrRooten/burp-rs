use std::fs;

use log::error;
use rutie::AnyObject;

use crate::{
    modules::{IActive, ModuleMeta},
    utils::STError, st_error, libruby::utils::{get_instance, call_object_method, object_to_string},
};

pub struct RBModule {
    module_script: String,
    object: AnyObject,
}

impl RBModule {
    pub fn new(file: &str) -> Result<Self, STError> {
        Ok(Self {
            module_script: file.to_string(),
            object : get_instance(file, "RBModule", &[])
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
        let result = call_object_method(&self.object, "metadata", &[]).unwrap();
        let result = call_object_method(&result, "to_json", &[]).unwrap();
        let result = call_object_method(&result, "to_s", &[]).unwrap();
        let s = object_to_string(&result).unwrap();
        println!("{}",s);
        None
    }
}
