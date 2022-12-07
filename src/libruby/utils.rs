use std::fs;

use rutie::{VM, eval, Object, Binding, RString, Class, AnyObject};

use crate::{utils::STError, st_error};

pub fn init() -> Result<(), STError> {
    VM::init();
    VM::init_loadpath();
    unimplemented!()
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

pub fn call_method(script: &str, class: &str, method: &str, arguments: &[AnyObject]) -> Result<AnyObject, STError> {
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
    Class::from_existing(class).new_instance(arguments)
}