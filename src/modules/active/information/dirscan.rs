use std::{str::FromStr, sync::Arc, fs::{File}, io::{BufReader, BufRead}};

use colored::Colorize;
use hyper::{Method, StatusCode, Uri};
use log::{error, info};
use strsim::normalized_levenshtein;
use tokio::sync::Semaphore;

use crate::{
    librs::http::utils::HttpRequest,
    modules::{IActive, ModuleMeta, ModuleType},
    st_error,
    utils::{STError, config::{get_config}}, proxy::log::{SiteMap, FoundUrl},
};

pub struct DirScan {
    meta: Option<ModuleMeta>,
}



fn dir_scan(url: &str) -> Result<Vec<crate::modules::Issue>, STError> {
    let mut found_urls = Vec::new();
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
    let not_found_request = match HttpRequest::from_url(&not_found_url) {
        Ok(o) => o,
        Err(e) => {
            return Err(e);
        }
    };
    let resp = HttpRequest::send(Method::GET, &not_found_request);
    let resp = match resp {
        Ok(o) => o,
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    };
    let not_found_content_s = Arc::new(String::from_utf8_lossy(resp.get_body()).to_string());

    let dict: Vec<String> = vec!["index.html".to_string(),"index.php".to_string()];
    let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(o) => o,
        Err(e) => {
            return Err(st_error!(e));
        }
    };

    let mut asyncs = vec![];
    let mut count = 0 ;
    let config = get_config();
    let num_parallel = config.get("modules.dir_scan.parallel").as_i64();
    let dict_path = config.get("modules.dir_scan.wordlist").as_str();
    let num_parallel = match num_parallel {
        Some(s) => s,
        None => {
            3
        }
    };

    let dict_path = match dict_path {
        Some(s) => s,
        None => {
            return Err(STError::new("No wordlist file"));
        }
    };
    let f = match File::open(dict_path) {
        Ok(o) => o,
        Err(e) => {
            return Err(st_error!(e));
        }
    };
    let lines = BufReader::new(f).lines();


    let sem = Arc::new(Semaphore::new(num_parallel as usize));
    for item in lines {
        let url = base_url.clone();
        let not_found = not_found_content_s.clone();
        let sem_clone = sem.clone();
        let item = match item {
            Ok(o) => o,
            Err(e) => {
                return Err(st_error!(e));
            }
        };
        count += 1;
        if count % 121 == 0 {
            info!("Currently have scan {} url", count);
        }
        let ret = rt.spawn(async move {
            let target_url = format!("{}{}", url, item);
            let request = match HttpRequest::from_url(&target_url) {
                Ok(e) => e,
                Err(e) => {
                    return None::<FoundUrl>;
                }
            };
            let aq = sem_clone.acquire().await;
            let resp =HttpRequest::send_async(Method::GET, &request).await;
            
            let resp = match resp {
                Ok(o) => o,
                Err(e) => {
                    error!("{}", e);
                    return None::<FoundUrl>;
                }
            };

            if resp.get_status().eq(&StatusCode::NOT_FOUND)
                || resp.get_status().eq(&StatusCode::FORBIDDEN)
            {
                return None::<FoundUrl>;
            }

            let content = String::from_utf8_lossy(resp.get_body()).to_string();
            //r.push("sdf".to_string());
            let result = normalized_levenshtein(&content, &not_found.clone());
            if result > 0.9 {
                return None::<FoundUrl>;
            }
            let length: u32;
            if resp.get_header("content-length").len() == 0 {
                length = 0;
            } else {
                length = match resp.get_header("content-length").parse::<u32>() {
                    Ok(o) => o,
                    Err(e) => {
                        error!("{}", e);
                        0
                    }
                };
            }
            let ret = FoundUrl::new(
                Method::GET,
                &target_url,
                length,
                resp.get_status().as_u16(),
                &resp.get_header("content-type")
            );
            Some(ret)
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
    let map = match SiteMap::single() {
        Some(s) => s,
        None => {
            return Err(STError::new("Error to get sitemap"));
        }
    };
    info!("Found {} urls is accessiable, use sitemap ${{host_key}} to checkout", found_urls.len().to_string().red());
    for i in found_urls {
        map.add_exist_path(&i);
    }

    Ok(vec![])
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
        //println!("test");
        let result = dir_scan(url);
        return Ok(vec![]);
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
