use std::{fs, sync::Arc};

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
    config      : Vec<Yaml>,
    proxy       : Arc<String>
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
        let path = "http.proxy";
        let keys = path.split(".").collect::<Vec<&str>>();

        let mut sub: &Yaml = &yaml[0];
        for key in keys {
            sub = &sub[key];
        }

        let proxy = match sub.as_str() {
            Some(s) => s,
            None => ""
        };

        let proxy = proxy.to_string();
        Ok(Self {
            config  : yaml,
            proxy: Arc::new(proxy)
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

    pub fn get_proxy(&self) -> Arc<String> {
        self.proxy.clone()
    }
}