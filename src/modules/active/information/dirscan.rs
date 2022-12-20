use std::{str::FromStr, sync::Arc};

use hyper::{Method, StatusCode, Uri};
use log::error;
use strsim::normalized_levenshtein;

use crate::{
    librs::http::utils::HttpRequest,
    modules::{IActive, ModuleMeta, ModuleType},
    st_error,
    utils::STError,
};

pub struct DirScan {
    meta: Option<ModuleMeta>,
}

fn dir_scan(url: &str) -> Result<Vec<crate::modules::Issue>, STError> {
    let mut found_urls = Vec::<String>::new();
    let url = Uri::from_str(url);
    let url = match url {
        Ok(o) => o,
        Err(e) => {
            let s = format!("{}", e);
            return Err(STError::new(&s));
        }
    };

    let base_url: String;
    if url.port().is_none() {
        base_url = format!("{}://{}/", url.scheme_str().unwrap(), url.host().unwrap());
    } else {
        base_url = format!(
            "{}://{}:{}/",
            url.scheme_str().unwrap(),
            url.host().unwrap(),
            url.port().unwrap()
        );
    }
    let not_found_url = format!("{}fjaskdfbasjdkhfasdjfhbvjasdfhjsadh", base_url);
    let not_found_request = HttpRequest::from_url(&not_found_url);
    let resp = HttpRequest::send(Method::GET, &not_found_request);
    let resp = match resp {
        Ok(o) => o,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    };
    let not_found_content_s = Arc::new(String::from_utf8_lossy(resp.get_body()).to_string());

    let dict: Vec<String> = vec![];
    let rt = match tokio::runtime::Builder::new_current_thread().build() {
        Ok(o) => o,
        Err(e) => {
            return Err(st_error!(e));
        }
    };

    let mut asyncs = vec![];
    for item in dict {
        let url = base_url.clone();
        let ret = rt.spawn(async move {
            let target_url = format!("{}{}", url, item);
            let request = HttpRequest::from_url(&target_url);
            let resp = HttpRequest::send_async(Method::GET, &request).await;
            let resp = match resp {
                Ok(o) => o,
                Err(e) => {
                    error!("{}", e);
                    return None::<String>;
                }
            };

            if resp.get_status().eq(&StatusCode::NOT_FOUND)
                || resp.get_status().eq(&StatusCode::FORBIDDEN)
            {
                return None::<String>;
            }

            let content = String::from_utf8_lossy(resp.get_body()).to_string();
            //r.push("sdf".to_string());
            unimplemented!()
        });

        asyncs.push(ret);
    }

    let found_urls = rt.block_on(async move {
        for i in asyncs {
            let url = match i.await {
                Ok(o) => o,
                Err(e) => {
                    error!("{}", e);
                    continue;
                }
            };

            let url = match url {
                Some(u) => u,
                None => {
                    continue;
                }
            };

            found_urls.push(url);
        }

        found_urls
    });
    unimplemented!()
}
impl IActive for DirScan {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>, crate::utils::STError> {
        let result = vec![];

        return Ok(result);
    }

    fn active_run(
        &self,
        url: &str,
        args: crate::modules::Args,
    ) -> Result<Vec<crate::modules::Issue>, crate::utils::STError> {
        let result = dir_scan(url);
        return result;
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

impl DirScan {
    pub fn new() -> Self {
        let meta = ModuleMeta {
            name: "dir_scan".to_string(),
            description: "Scan url".to_string(),
            m_type: ModuleType::RustModule,
        };
        Self { meta: Some(meta) }
    }
}
