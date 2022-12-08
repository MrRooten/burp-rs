use std::fs;

use log::error;
use rutie::{AnyObject, RString, Object};
use serde_json::Value;

use crate::{
    modules::{IActive, ModuleMeta},
    utils::STError, st_error, libruby::utils::{get_instance, call_object_method, object_to_string},
};

pub struct RBModule {
    module_script: String,
    object: AnyObject,
    passive_method  : AnyObject,
    meta_method     : AnyObject,
    active_method   : AnyObject
}

impl RBModule {
    pub fn new(file: &str) -> Result<Self, STError> {
        let object = get_instance(file, "RBModule", &[]);
        let passive_str = RString::from("passive_run");
        let arg1: AnyObject = passive_str.try_convert_to::<AnyObject>().unwrap();
        let passive_method = call_object_method(&object, "method", &[arg1]).unwrap();

        let metadata_str = RString::from("metadata");
        let arg1: AnyObject = metadata_str.try_convert_to::<AnyObject>().unwrap();
        let metadata_method = call_object_method(&object, "method", &[arg1]).unwrap();

        let active_str = RString::from("active_run");
        let arg1: AnyObject = metadata_str.try_convert_to::<AnyObject>().unwrap();
        let active_method = call_object_method(&object, "method", &[arg1]).unwrap();
        Ok(Self {
            module_script: file.to_string(),
            object : object,
            passive_method : passive_method,
            meta_method    : metadata_method,
            active_method   : active_method
        })
    }
}

impl IActive for RBModule {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>, STError> {
        
        let result = self.passive_method.protect_send("call", &[]).unwrap();
        Ok(vec![])
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
        let ret: Value = serde_json::from_str(&s).unwrap();
        let name = ret.get("name").unwrap().as_str().unwrap();
        let description = ret.get("description").unwrap().as_str().unwrap();
        let meta = ModuleMeta {
            name: name.to_string(),
            description: description.to_string(),
        };
        println!("{:?}",meta);
        None
    }
}
