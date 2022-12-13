use hudsucker::{
    async_trait::async_trait, certificate_authority::RcgenAuthority,
    tokio_tungstenite::tungstenite::Message, *,
};
use rustls_pemfile as pemfile;


use crate::{
    modules::passive::PassiveScanner,
    proxy::{
        filter::{is_capture_req, is_capture_res},
        log::{LogHistory, LogRequest, LogResponse, ReqResLog, SiteMap},
    },
};
use hyper::{
    body::{self, Bytes},
    Body, Method, Request, Response,
};
use std::{net::SocketAddr, sync::mpsc::{Receiver, Sender, self, SyncSender}, thread::spawn};
use tracing::*;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

#[derive(Clone, Default)]
struct ProxyHandler {
    index: u32,
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
    return LogRequest::from(new_req, s);
}

async fn copy_req_header(req: &mut Request<Body>) -> LogRequest {
    let body = req.body_mut();
    let s = body::to_bytes(body).await.unwrap();
    let mut new_req = Request::new(Body::from(""));
    *req.body_mut() = Body::from(s.clone());
    new_req.headers_mut().clone_from(req.headers());
    new_req.method_mut().clone_from(req.method());
    new_req.uri_mut().clone_from(req.uri());
    new_req.version_mut().clone_from(&req.version());
    new_req.extensions().clone_from(&req.extensions());
    return LogRequest::from(new_req, s);
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
    return LogResponse::from(new_res, s);
}

async fn copy_resp_header(resp: &mut Response<Body>) -> LogResponse {
    let body = resp.body_mut();
    let mut new_res = Response::new(Body::from(""));
    new_res.extensions().clone_from(&resp.extensions());
    new_res.headers_mut().clone_from(resp.headers());
    new_res.status_mut().clone_from(&resp.status());
    new_res.version_mut().clone_from(&resp.version());
    return LogResponse::from(new_res, Bytes::new());
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

        //println!("{:?}", req);
        if is_capture_req(&req) {
            let log = copy_req(&mut req).await;
            let reqres_log = ReqResLog::new(log);
            self.index = match history.push_log(reqres_log) {
                Ok(index) => index,
                Err(e) => self.index,
            }
        } else {
            let log = copy_req_header(&mut req).await;
            let reqres_log = ReqResLog::new(log);
            self.index = match history.push_log(reqres_log) {
                Ok(index) => index,
                Err(e) => self.index,
            }
        }
        let s = SiteMap::single();
        RequestOrResponse::Request(req)
    }

    async fn handle_response(
        &mut self,
        _ctx: &HttpContext,
        mut res: Response<Body>,
    ) -> Response<Body> {
        let history = LogHistory::single();
        let history = match history {
            Some(h) => h,
            None => {
                return res;
            }
        };
        //println!("{:?}", res);
        if is_capture_res(&res) {
            let res_log = copy_resp(&mut res).await;
            history.set_resp(self.index, res_log);
        } else {
            let res_log = copy_resp_header(&mut res).await;
            history.set_resp(self.index, res_log);
        }
        let index = self.index.clone();
        unsafe {
            let sender = &mut PASSIVE_SCAN_SENDER;
            let sender = match sender {
                Some(o) => o,
                None => {
                    //Httplog does not have 0 index
                    panic!("Sender Panic")
                }
            };

            match sender.send(index) {
                Ok(o) => o,
                Err(e) => {
                    error!("{}",e);
                }
            };
        }
        res
    }
}

#[async_trait]
impl WebSocketHandler for ProxyHandler {
    async fn handle_message(&mut self, _ctx: &WebSocketContext, msg: Message) -> Option<Message> {
        //println!("{:?}", msg);
        Some(msg)
    }
}

pub static mut PASSIVE_SCAN_SENDER: Option<std::sync::mpsc::SyncSender<u32>> = None::<SyncSender<u32>>;
pub static mut PASSIVE_SCAN_RECEIVER: Option<std::sync::mpsc::Receiver<u32>> =
    None::<Receiver<u32>>;

pub async fn proxy(addr: &str) {
    unsafe {
        if PASSIVE_SCAN_SENDER.is_none() || PASSIVE_SCAN_RECEIVER.is_none() {
            let (tx, rx) = mpsc::sync_channel(0);
            PASSIVE_SCAN_SENDER = Some(tx);
            PASSIVE_SCAN_RECEIVER = Some(rx);
        }
        //Async way to passively scan
        spawn(|| {
            let receiver = &mut PASSIVE_SCAN_RECEIVER;
            let receiver = match receiver {
                Some(o) => o,
                None => {
                    //Httplog does not have 0 index
                    panic!("Receiver")
                }
            };
            let scanner = PassiveScanner::new();
            loop {
                let index = receiver.recv();
                let index = match index {
                    Ok(i) => i,
                    Err(e) => {
                        error!("{}",e);
                        continue;
                    }
                };
                scanner.passive_scan(index);
            }
        });
    }

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
