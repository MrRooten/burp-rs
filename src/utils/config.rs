use std::fs;

use yaml_rust::YamlLoader;

use super::STError;

pub struct Config {

}

impl Config {
    pub fn read() -> Result<Self,STError>{
        let config_s = fs::read_to_string("../../config.yaml").unwrap();
        let yaml = YamlLoader::load_from_str(&config_s).unwrap();
        let config = &yaml[0];
        println!("{:?}",config);
        unimplemented!()
    }
}