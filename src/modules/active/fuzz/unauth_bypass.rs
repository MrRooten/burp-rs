use std::{str::FromStr, vec};

use hyper::{Uri, Method};

use crate::{modules::{IActive, ModuleMeta, ModuleType}, librs::http::utils::HttpRequest, st_error};

pub struct UnauthBypass {
    meta: Option<ModuleMeta>,
}

fn run(method: Method, url: &str) -> Result<Vec<crate::modules::Issue>,crate::utils::STError> {
    let payloads = vec![
        "%09",
        "%20" ,
        "%23" ,
        "%2e" ,
        "%2f" ,
        "." ,
        ";" ,
        "..;" ,
        ";%09" ,
        ";%09.." ,
        ";%09..;" ,
        ";%2f.." ,
        "*" ,
        "HTTPS2"
    ];
    
    let uri = match Uri::from_str(url) {
        Ok(u) => u,
        Err(e) => {
            return Err(st_error!(e));
        }
    };
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    let mut nodes = uri.path().split("/").collect::<Vec<&str>>();
    let mut i = 0;
    let mut handles = vec![];
    for node in nodes {
        let var_node = node.to_string();
        for payload in payloads {
            let out = format!("{}/{}", var_node, payload);
            nodes[i] = &out;
            let payload_path = nodes.join("/");
            let target_u = match uri.port() {
                Some(o) => {
                    format!("{}://{}:{}/{}?{}",uri.scheme_str().unwrap(), 
                    uri.host(),o,payload_path,uri.query().unwrap_or(""))
                },
                None => {
                    format!("{}://{}/{}?{}",uri.scheme_str().unwrap(), 
                    uri.host(),payload_path,uri.query().unwrap_or(""))
                }
            };

            let request = match HttpRequest::from_url(&target_u) {
                Ok(o) => o,
                Err(e) => {
                    return Err(e);
                }
            };

            let handle = rt.spawn(async move {
                let resp = HttpRequest::send_async(method, &request).await;
            });
            handles.push(handle);
        }
    }

    rt.block_on(async move {
        for h in handles {
            h.await
        }
    });

    Ok(vec![])
}

impl IActive for UnauthBypass {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>,crate::utils::STError> {
        let result = Vec::default();
        println!("passive_run...");
        return Ok(result);
    }

    fn active_run(&self, url: &str, args: crate::modules::Args) -> Result<Vec<crate::modules::Issue>,crate::utils::STError> {
        let result = Vec::default();
        println!("passive_run...");
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

impl UnauthBypass {
    pub fn new() -> Self {
        let meta = ModuleMeta {
            name: "unauth_bypass".to_string(),
            description: "403 bypasser".to_string(),
            m_type: ModuleType::RustModule,
        };
        Self { meta: Some(meta) }
    }
}