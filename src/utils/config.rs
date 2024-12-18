use std::{fs, ptr::addr_of_mut, sync::Arc};

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
        &mut*addr_of_mut!(CONFIG)
    }
}



pub struct ArgsConfig {
    ca_path: Option<String>
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
        let keys = path.split('.').collect::<Vec<&str>>();

        let mut sub: &Yaml = &yaml[0];
        for key in keys {
            sub = &sub[key];
        }

        let proxy = sub.as_str().unwrap_or("");

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
        let keys = path.split('.').collect::<Vec<&str>>();
        let doc = &self.config[0];
        let mut sub: &Yaml = doc;
        for key in keys {
            sub = &sub[key];
        }

        sub
    }

    pub fn set(&mut self, path: &str, value: &str) -> Result<(), STError> {
        let keys = path.split('.').collect::<Vec<&str>>();
        let doc = &self.config[0];
        let mut sub: &Yaml = doc;
        for key in keys {
            sub = &sub[key];
        }
        
        unimplemented!()
    }

    pub fn get_proxy(&self) -> Arc<String> {
        self.proxy.clone()
    }
}


use rcgen::*;


pub fn generate_key() {
    match std::fs::create_dir("./ca") {
        Ok(o) => (),
        Err(e) => {
            println!("{}", e);
        }
    }

    let mut params = CertificateParams::default();
    let mut distinguished_name = DistinguishedName::new();

    distinguished_name.push(DnType::CommonName, "KK");
    distinguished_name.push(DnType::OrganizationName, "KK");
    distinguished_name.push(DnType::CountryName, "US");
    distinguished_name.push(DnType::StateOrProvinceName, "NY");
    distinguished_name.push(DnType::LocalityName, "NYC");

    params.distinguished_name = distinguished_name;
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.key_usages = vec![
        KeyUsagePurpose::KeyCertSign,
        KeyUsagePurpose::CrlSign,
    ];

    let cert = Certificate::from_params(params).unwrap();
    let private_key = cert.serialize_private_key_pem();

    fs::write("ca/cert.pem", cert.serialize_pem().unwrap()).unwrap();
    fs::write("ca/private.key", private_key).unwrap();
}