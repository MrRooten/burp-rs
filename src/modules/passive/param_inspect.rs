use crate::{modules::IPassive, utils::STError, proxy::log::LogHistory};

pub struct ParamInspect;

pub fn is_base64(s: &str) -> bool {
    false
}

pub fn is_jave_deserialize(s: &str) -> bool {
    false
}


impl IPassive for ParamInspect {
    fn run(&self, index: u32) -> Result<(), crate::utils::STError> {
        let log = LogHistory::get_httplog(index);
        let log = match log {
            Some(o) => o,
            None => {
                return Err(STError::new("Not found history log"));
            }
        };

        let request = match log.get_request() {
            Some(r) => r,
            None => {
                return Err(STError::new("Not found history log request"));
            }
        };

        let params = request.get_params();
        for param in params {
            let v = param.get_value();
            if is_base64(v) {

            }

            if is_jave_deserialize(v) {

            }


        }
        unimplemented!()
    }

    fn name(&self) -> String {
        "param_inspect".to_string()
    }

    fn help(&self) -> crate::modules::Helper {
        todo!()
    }
}