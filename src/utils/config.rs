use std::fs;

use log::info;
use once_cell::sync::Lazy;
use yaml_rust::{YamlLoader, Yaml};

use crate::st_error;

use super::STError;

static mut CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::read().unwrap()
});

pub fn get_config() -> &'static mut Lazy<Config> {
    unsafe {
        &mut CONFIG
    }
}
pub struct Config {
    config      : Vec<Yaml>
}

impl Config {
    pub fn read() -> Result<Self,STError> {
        info!("Reading config from config.yaml");
        let config_s = match fs::read_to_string("./config.yaml") {
            Ok(o) => o,
            Err(e) => {
                return Err(st_error!(e));
            }
        };
        let yaml = match YamlLoader::load_from_str(&config_s) {
            Ok(o) => o,
            Err(e) => {
                return Err(st_error!(e));
            }
        };
        Ok(Self {
            config  : yaml
        })
    }

    pub fn update(&mut self) -> Result<(),STError> {
        info!("Reading config from config.yaml");
        let config_s = match fs::read_to_string("./config.yaml") {
            Ok(o) => o,
            Err(e) => {
                return Err(st_error!(e));
            }
        };
        let yaml = match YamlLoader::load_from_str(&config_s) {
            Ok(o) => o,
            Err(e) => {
                return Err(st_error!(e));
            }
        };
        self.config = yaml;
        Ok(())
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
        let keys = path.split(".").collect::<Vec<&str>>();
        let doc = &self.config[0];
        let mut sub: &Yaml = &doc;
        for key in keys {
            sub = &sub[key];
        }
        
        unimplemented!()
    }

}