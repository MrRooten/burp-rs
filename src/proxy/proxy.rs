use hudsucker::{
    async_trait::async_trait,
    certificate_authority::RcgenAuthority,
    tokio_tungstenite::tungstenite::Message,
    *,
};
use rustls_pemfile as pemfile;

use std::net::SocketAddr;
use tracing::*;
use crate::proxy::{log::{LogHistory, ReqResLog, LogResponse, LogRequest, SiteMap}, filter::{is_capture_res, is_capture_req}};
use hyper::{Body, Request, Response, body::{self}, Method};

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

#[derive(Clone,Default)]
struct ProxyHandler {
    index    : u32,
}

async fn copy_req(req: &mut Request<Body>) -> LogRequest {
    let body = req.body_mut();
    
    let s = body::to_bytes(body).await.unwrap();
    let mut new_req = Request::new(Body::from(""));
    *req.body_mut() = Body::from(s.clone());
    new_req.headers_mut().clone_from(req.headers());
    new_req.method_mut().clone_from(req.method());
    new_req.uri_mut().clone_from(req.uri());
    new_req.version_mut().clone_from(&req.version());
    new_req.extensions().clone_from(&req.extensions());
    return LogRequest::from(new_req,s);
}

async fn copy_resp(resp: &mut Response<Body>) -> LogResponse {
    let body = resp.body_mut();
    let s = body::to_bytes(body).await.unwrap();
    let mut new_res = Response::new(Body::from(""));
    *resp.body_mut() = Body::from(s.clone());
    new_res.extensions().clone_from(&resp.extensions());
    new_res.headers_mut().clone_from(resp.headers());
    new_res.status_mut().clone_from(&resp.status());
    new_res.version_mut().clone_from(&resp.version());
    return LogResponse::from(new_res,s);
}
#[async_trait]
impl HttpHandler for ProxyHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        mut req: Request<Body>,
    ) -> RequestOrResponse {
        if req.method() == Method::CONNECT {
            return RequestOrResponse::Request(req);
        }
        let history = LogHistory::single();
        let history = match history {
            Some(h) => h,
            None => {
                return RequestOrResponse::Request(req);
            }
        };
        let log = copy_req(&mut req).await;
        //println!("{:?}", req);
        if is_capture_req(&log) {
            let reqres_log = ReqResLog::new(log);
            let b = reqres_log.clone();
            self.index = history.push_log(reqres_log);
        }
        let s = SiteMap::single();
        RequestOrResponse::Request(req)
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, mut res: Response<Body>) -> Response<Body> {
        
        let res_log = copy_resp(&mut res).await;
 
        let history = LogHistory::single();
        let history = match history {
            Some(h) => h,
            None => {
                return res;
            }
        };
        //println!("{:?}", res);
        if is_capture_res(&res_log) {
            history.set_resp(self.index, res_log);
        }
        
        res
    }
}

#[async_trait]
impl WebSocketHandler for ProxyHandler {
    async fn handle_message(&mut self, _ctx: &WebSocketContext, msg: Message) -> Option<Message> {
        println!("{:?}", msg);
        Some(msg)
    }
}


pub async fn proxy(addr: &str) {
    let sock: SocketAddr = addr.parse().unwrap();
    let mut private_key_bytes: &[u8] = include_bytes!("../ca/rs.key");
    let mut ca_cert_bytes: &[u8] = include_bytes!("../ca/rs.cer");
    let private_key = rustls::PrivateKey(
        pemfile::pkcs8_private_keys(&mut private_key_bytes)
            .expect("Failed to parse private key")
            .remove(0),
    );
    let ca_cert = rustls::Certificate(
        pemfile::certs(&mut ca_cert_bytes)
            .expect("Failed to parse CA certificate")
            .remove(0),
    );

    let ca = RcgenAuthority::new(private_key, ca_cert, 1_000)
        .expect("Failed to create Certificate Authority");

    let proxy = Proxy::builder()
        .with_addr(sock)
        .with_rustls_client()
        .with_ca(ca)
        .with_http_handler(ProxyHandler::default())
        .with_websocket_handler(ProxyHandler::default())
        .build();

    if let Err(e) = proxy.start(shutdown_signal()).await {
        error!("{}", e);
    }
}
