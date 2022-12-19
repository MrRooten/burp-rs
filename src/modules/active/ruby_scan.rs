use std::{fs, io};

use rutie::{AnyObject, Exception, Integer, Object, RString, GC};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{
    libruby::utils::{call_object_method, get_instance, object_to_string},
    modules::{IActive, ModuleMeta, ModuleType},
    utils::STError,
};

pub struct RBModule {
    hash: String,
    module_script: String,
    object: AnyObject,
    passive_method: AnyObject,
    meta_method: AnyObject,
    active_method: AnyObject,
    meta: Option<ModuleMeta>,
}

impl RBModule {
    fn get_file_hash(file: &str) -> String {
        let mut hasher = Sha256::new();
        let mut file = fs::File::open(file).unwrap();

        let bytes_written = io::copy(&mut file, &mut hasher).unwrap();
        format!("{:X}", hasher.finalize())
    }

    pub fn new(file: &str) -> Result<Self, STError> {
        let object = get_instance(file, "RBModule", &[]);
        GC::register_mark(&object);
        let passive_str = RString::from("passive_run");
        let arg1: AnyObject = passive_str.try_convert_to::<AnyObject>().unwrap();
        let passive_method = match call_object_method(&object, "method", &[arg1]) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        GC::register_mark(&passive_method);
        let metadata_str = RString::from("metadata");
        let arg1: AnyObject = metadata_str.try_convert_to::<AnyObject>().unwrap();
        let metadata_method = match call_object_method(&object, "method", &[arg1]) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };

        let active_str = RString::from("active_run");
        let arg1: AnyObject = metadata_str.try_convert_to::<AnyObject>().unwrap();
        let active_method = match call_object_method(&object, "method", &[arg1]) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        GC::register_mark(&active_method);
        let result = metadata_method.protect_send("call", &[]).unwrap();
        let result = call_object_method(&result, "to_json", &[]).unwrap();
        let result = call_object_method(&result, "to_s", &[]).unwrap();

        let s = object_to_string(&result).unwrap();
        let ret: Value = serde_json::from_str(&s).unwrap();
        let name = ret.get("name").unwrap().as_str().unwrap();
        let description = ret.get("description").unwrap().as_str().unwrap();
        let meta = ModuleMeta {
            name: name.to_string(),
            description: description.to_string(),
            m_type  : ModuleType::RubyModule
        };
        //println!("{:?}",meta);
        Ok(Self {
            hash: Self::get_file_hash(file),
            module_script: file.to_string(),
            object: object,
            passive_method: passive_method,
            meta_method: metadata_method,
            active_method: active_method,
            meta: Some(meta),
        })
    }
}

impl IActive for RBModule {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>, STError> {
        let i = Integer::new(index.into())
            .try_convert_to::<AnyObject>()
            .unwrap();
        let result = match self.passive_method.protect_send("call", &[i]) {
            Ok(o) => o,
            Err(e) => {
                let v = match e.backtrace() {
                    Some(a) => {
                        a
                    },
                    None => {
                        let msg = format!("passive_run:{}", e);
                        return Err(STError::new(&msg));
                    }
                };
                let mut bt = vec![];
                for i in v {
                    bt.push(object_to_string(&i).unwrap_or_else(|x| { format!("{}",x) }));
                }
                let msg = format!("passive_run:{} \n{}",e.message(), bt.join("\n"));
                return Err(STError::new(&msg));
            }
        };
        Ok(vec![])
    }

    fn active_run(
        &self,
        url: &str,
        args: crate::modules::Args,
    ) -> Result<Vec<crate::modules::Issue>, STError> {
        todo!()
    }

    fn metadata(&self) -> &Option<ModuleMeta> {
        return &self.meta;
    }

    fn is_change(&self) -> bool {
        return Self::get_file_hash(&self.module_script).eq(&self.hash) == false;
    }

    fn update(&mut self) -> Result<(), STError>{
        GC::unregister(&self.object);
        GC::unregister(&self.active_method);
        GC::unregister(&self.passive_method);

        let object = get_instance(&self.module_script, "RBModule", &[]);
        GC::register_mark(&object);
        let passive_str = RString::from("passive_run");
        let arg1: AnyObject = passive_str.try_convert_to::<AnyObject>().unwrap();
        let passive_method = match call_object_method(&object, "method", &[arg1]) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        GC::register_mark(&passive_method);
        let metadata_str = RString::from("metadata");
        let arg1: AnyObject = metadata_str.try_convert_to::<AnyObject>().unwrap();
        let metadata_method = match call_object_method(&object, "method", &[arg1]) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };

        let active_str = RString::from("active_run");
        let arg1: AnyObject = metadata_str.try_convert_to::<AnyObject>().unwrap();
        let active_method = match call_object_method(&object, "method", &[arg1]) {
            Ok(o) => o,
            Err(e) => return Err(e),
        };
        GC::register_mark(&active_method);
        let result = metadata_method.protect_send("call", &[]).unwrap();
        let result = call_object_method(&result, "to_json", &[]).unwrap();
        let result = call_object_method(&result, "to_s", &[]).unwrap();

        let s = object_to_string(&result).unwrap();
        let ret: Value = serde_json::from_str(&s).unwrap();
        let name = ret.get("name").unwrap().as_str().unwrap();
        let description = ret.get("description").unwrap().as_str().unwrap();
        let meta = ModuleMeta {
            name: name.to_string(),
            description: description.to_string(),
            m_type: ModuleType::RubyModule
        };

        self.object = object;
        self.hash = Self::get_file_hash(&self.module_script);
        self.active_method = active_method;
        self.passive_method = passive_method;

        Ok(())
    }
}
