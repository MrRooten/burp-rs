use std::str::FromStr;

use hyper::{Uri, HeaderMap, http::HeaderValue, header::HOST};
use log::info;
use tracing::Metadata;

use crate::{modules::{IActive, ModuleMeta, ModuleType}, utils::STError};

pub struct TestScan {
    meta: Option<ModuleMeta>
}

impl IActive for TestScan {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>,crate::utils::STError> {
        let result = Vec::default();
        println!("passive_run...");
        return Ok(result);
    }

    fn active_run(&self, url: &str, args: crate::modules::Args) -> Result<Vec<crate::modules::Issue>,crate::utils::STError> {
        let result = vec![];
        let uri = match Uri::from_str(url) {
            Ok(o) => o,
            Err(e) => {
                let s = format!("{}",e);
                return Err(STError::new(&s));
            }
        };
        let host_with_port = match uri.port() {
            Some(port) => {
                format!("{}:{}", uri.host().unwrap(), port)
            }
            None => {
                format!("{}", uri.host().unwrap())
            }
        };
        let mut headers = HeaderMap::new();
        let host_key = HeaderValue::from_str("host").unwrap();
        headers.insert(HOST, host_with_port.parse().unwrap());
        let u = url.to_string();
        let h = headers.clone();
        info!("passive_run...");
        return Ok(result);
    }

    fn metadata(&self) -> &Option<crate::modules::ModuleMeta> {
        &self.meta
    }

    fn is_change(&self) -> bool {
        false
    }

    fn update(&mut self) -> Result<(), crate::utils::STError> {
        Ok(())
    }
}

impl TestScan {
    pub fn new() -> Self {
        let meta = ModuleMeta {
            name: "test".to_string(),
            description: "Test Module in passive".to_string(),
            m_type: ModuleType::RustModule,
        };
        Self {
            meta : Some(meta)
        }
    }
}