use std::sync::Arc;

use crate::{modules::IPassive, proxy::log::ReqResLog, librs::http::utils::BurpRequest};

pub struct SerializeDetect;

impl IPassive for SerializeDetect {
    fn run(&self, log: &Arc<ReqResLog>, burp: &BurpRequest) -> Result<(), crate::utils::STError> {
        todo!()
    }

    fn name(&self) -> String {
        "SerializeDetect".to_string()
    }

    fn help(&self) -> crate::modules::Helper {
        todo!()
    }
}