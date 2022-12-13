use std::{fs};

use rutie::{VM, eval, Object, Binding, RString, Class, AnyObject};

use crate::{utils::STError, st_error};

use super::{http::{log::{get_http_req, get_http_resp}, utils::send}, log::{error, debug, info, warn}, issue::push_issue};

pub fn rb_init() -> Result<(), STError> {
    VM::init();
    VM::init_loadpath();
    Class::new("RBHttpLog", None).define(|klass| {
        klass.def("get_http_req", get_http_req);
        klass.def("get_http_resp", get_http_resp);
    });

    Class::new("RBHttpClient", None).define(|klass| {
        klass.def("send", send);
    });

    Class::new("RBLogger", None).define(|klass| {
        klass.def("error",error);
        klass.def("debug",debug);
        klass.def("info",info);
        klass.def("warn",warn);
    });

    Class::new("RBIssue", None).define(|klass| {
        klass.def("push_issue", push_issue);
    });
    VM::require("json");
    VM::require("enc/encdb");
    VM::require("enc/trans/transdb");
    VM::require("./active/http/http");
    VM::require("./active/http/log");
    VM::require("./active/logger/logger");
    
    Ok(())
}

pub fn call_func(script: &str, func: &str) -> Result<(),STError> {
    let target = fs::read_to_string(script);
    let target = match target {
        Ok(o) => o,
        Err(e) => {
            return Err(st_error!(e));
        }
    };
    let target = RString::from(target);
    let binding = r#"
    def get_binding
        return binding
    end
    "#;
    let binding = eval!(binding).unwrap().try_convert_to::<Binding>().unwrap();
    let result = eval!(target, binding).unwrap();

    unimplemented!()
}

pub fn call_class_object_method(script: &str, class: &str, method: &str, arguments: &[AnyObject]) -> Result<AnyObject, STError> {
    let f = fs::read_to_string(script);
    let s = match f {
        Ok(o) => o,
        Err(e) => {
            return Err(st_error!(e));
        }
    };
    let _ = eval!(&s);
    let module = Class::from_existing(class).new_instance(&[]);
    let ret = module.protect_send(method, arguments);
    let ret_obj = match ret {
        Ok(o) => o,
        Err(e) => {
            let s = format!("{:?}",e);
            return Err(STError::new(&s));
        }
    };
    Ok(ret_obj)
}

pub fn get_instance(script: &str, class: &str, arguments: &[AnyObject]) -> AnyObject {
    //let _ = eval!(&fs::read_to_string(script).unwrap());
    let path = format!("./{}",script);
    VM::require(&path);
    Class::from_existing(class).new_instance(arguments)
}

pub fn call_object_method(object: &AnyObject, method: &str, arguments: &[AnyObject]) -> Result<AnyObject, STError> {
    let ret = object.protect_send(method, arguments);
    let ret_obj = match ret {
        Ok(o) => o,
        Err(e) => {
            let s = format!("{:?}",e);
            return Err(STError::new(&s));
        }
    };
    Ok(ret_obj)
}

pub fn object_to_json(object: &AnyObject) -> Result<String, STError> {
    let ret = object.protect_send("to_json", &[]);
    let ret_obj = match ret {
        Ok(o) => o,
        Err(e) => {
            let s = format!("{:?}",e);
            return Err(STError::new(&s));
        }
    };

    let ret = match ret_obj.try_convert_to::<RString>() {
        Ok(o) => o.to_string(),
        Err(e) => {
            let s = format!("{:?}",e);
            return Err(STError::new(&s));
        } 
    };


    Ok(ret)
}

pub fn object_to_string(object: &AnyObject) -> Result<String, STError> {
    let ret = object.protect_send("to_s", &[]);
    let ret_obj = match ret {
        Ok(o) => o,
        Err(e) => {
            let s = format!("{:?}",e);
            return Err(STError::new(&s));
        }
    };

    let ret = match ret_obj.try_convert_to::<RString>() {
        Ok(o) => o.to_string(),
        Err(e) => {
            let s = format!("{:?}",e);
            return Err(STError::new(&s));
        } 
    };


    Ok(ret)
}

