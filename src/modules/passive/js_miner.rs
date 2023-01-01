use std::{str::FromStr, sync::Arc};

use regex::Regex;

use crate::{modules::IPassive, proxy::log::{ReqResLog}, utils::STError, librs::http::utils::BurpRequest};


pub struct JsMiner;
static CLOUD_URLS_REGEX: &str = "([\\w]+[.]){1,10}(s3.amazonaws.com|\
    rds.amazonaws.com|cache.amazonaws.com|blob.core.windows.net|\
    onedrive.live.com|1drv.com|storage.googleapis.com|storage.cloud.google.com|\
    storage-download.googleapis.com|content-storage-upload.googleapis.com|\
    content-storage-download.googleapis.com\
    |cloudfront.net|digitaloceanspaces.com|oraclecloud.com|aliyuncs.com|aliyuncs.com|ackcdn.com|objects.cdn.dream.io|objects-us-west-1.dream.io)";

    static ECRETS_REGEX: &str = "\\w+(secret|token|password|passwd|authorization|bearer|aws_access_key_id|aws_secret_access_key|irc_pass|SLACK_BOT_TOKEN|id_dsa|\
        secret[_-]?(key|token|secret)|\
        api[_-]?(key|token|secret)|\
        access[_-]?(key|token|secret)|\
        auth[_-]?(key|token|secret)|\
        session[_-]?(key|token|secret)|\
        consumer[_-]?(key|token|secret)|\
        public[_-]?(key|token|secret)|\
        client[_-]?(id|token|key)|\
        ssh[_-]?key|\
        encrypt[_-]?(secret|key)|\
        decrypt[_-]?(secret|key)|\
        github[_-]?(key|token|secret)|\
        slack[_-]?token) ?= ?\"([\\w\\-/~!@#$%^&*+]+)\"";

impl IPassive for JsMiner {
    fn run(&self, log: &Arc<ReqResLog>, burp: &BurpRequest) -> Result<(), crate::utils::STError> {
        let screts_regex = Regex::from_str(ECRETS_REGEX);

        let response = match log.get_response() {
            Some(r) => r,
            None => {
                return Err(STError::new("Not found history log request"));
            }
        };
        let header = response.get_header("content-type");
        if header.is_none() {
            return Ok(());
        }


        let header = header.unwrap();
        if header.contains("javascript") == false {
            return Ok(());
        }


        Ok(())
    }

    fn name(&self) -> String {
        "js_miner".to_string()
    }

    fn help(&self) -> crate::modules::Helper {
        todo!()
    }
}