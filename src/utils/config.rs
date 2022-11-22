use std::fs;

use log::info;
use yaml_rust::YamlLoader;

use super::STError;

#[derive(Default)]
pub struct Config {
    request_deny_ext    : Option<String>,
    response_filter     : Option<String>,
}

impl Config {
    pub fn read() -> Result<Self,STError> {
        let mut result = Config::default();
        info!("Reading config from config.yaml");
        let config_s = fs::read_to_string("../../config.yaml").unwrap();
        let yaml = YamlLoader::load_from_str(&config_s).unwrap();
        let config = &yaml[0];
        println!("{:?}",config);
        result.request_deny_ext = match config["request_fileter"]["deny_ext"].as_str() {
            Some(s) => Some(s.to_string()),
            None => {
                None
            }
        };

        
        unimplemented!()
    }
}