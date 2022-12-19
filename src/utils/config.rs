use std::fs;

use log::info;
use yaml_rust::{YamlLoader, Yaml, yaml};

use super::STError;


pub struct Config {
    config      : Vec<Yaml>
}

impl Config {
    pub fn read() -> Result<Self,STError> {
        info!("Reading config from config.yaml");
        let config_s = fs::read_to_string("./config.yaml").unwrap();
        let yaml = YamlLoader::load_from_str(&config_s).unwrap();
        Ok(Self {
            config  : yaml
        })
    }

    pub fn get(&self, path: &str) -> &Yaml {
        let keys = path.split(".").collect::<Vec<&str>>();
        let doc = &self.config[0];
        let mut sub: &Yaml = &doc;
        for key in keys {
            sub = &sub[key];
        }

        sub
    }

    pub fn set(&mut self, path: &str, value: &str) -> Result<(), STError> {
        unimplemented!()
    }

}