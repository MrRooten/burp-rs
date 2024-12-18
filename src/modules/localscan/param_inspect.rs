use std::{sync::Arc, borrow::Cow, num::ParseIntError};
use urlencoding::decode;

use crate::{modules::{IPassive, Issue, IssueLevel, IssueConfidence}, proxy::log::ReqResLog, librs::http::utils::{BurpRequest, BurpParam}};

pub struct ParamInspect;

pub fn is_base64(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    match base64::decode(s) {
        Ok(o) => {
            true
        },
        Err(e) => {
            false
        }
    }
}

pub fn is_jave_deserialize(s: &Vec<u8>) -> bool {
    let _ = s;
    false
}

pub fn is_php_deserialize(s: &Vec<u8>) -> bool {
    let _ = s;
    false
}

pub fn is_dotnet_deserialize(s: &Vec<u8>) -> bool {
    let _ = s;
    false
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

impl IPassive for ParamInspect {
    fn run(&self, log: &Arc<ReqResLog>, burp: &BurpRequest, params: &Vec<BurpParam>) -> Result<(), crate::utils::STError> {
        for param in params {
            let v = param.get_value();
            let v = match decode(v) {
                Ok(o) => o,
                Err(e) => {
                    Cow::from(v)
                }
            };
            if is_base64(&v) {
                let detail = format!("base64 param: {}:{}", param.get_name(), v);
                let issue = Issue::new(
                    "Base64 param",
                    IssueLevel::Info,
                    &detail,
                    IssueConfidence::Confirm,
                    &log.get_host()
                );
                Issue::add_issue(issue, log);
                let data = base64::decode(&*v);
                if let Ok(o) = data {
                    is_jave_deserialize(&o);

                    if is_php_deserialize(&o) {

                    }


                }
            } 
        }
        Ok(())
    }

    fn name(&self) -> String {
        "param_inspect".to_string()
    }

    fn help(&self) -> crate::modules::Helper {
        todo!()
    }
}