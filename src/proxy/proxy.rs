use hudsucker::{
    async_trait::async_trait,
    certificate_authority::RcgenAuthority,
    hyper::{Body, Request, Response, body},
    tokio_tungstenite::tungstenite::Message,
    *,
};
use rustls_pemfile as pemfile;
use std::net::SocketAddr;
use tracing::*;
use crate::proxy::log::{LogHistory, ReqResLog, LogResponse};

use super::log::HTTP_LOG;
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

#[derive(Clone,Default)]
struct ProxyHandler {
    index    : u32,
}

#[async_trait]
impl HttpHandler for ProxyHandler {
    async fn handle_request(
        &mut self,
        _ctx: &HttpContext,
        req: Request<Body>,
    ) -> RequestOrResponse {
        println!("{:?}", req);
        let mut history = &LogHistory::single().unwrap();
        let log = ReqResLog::new();
        self.index = history.push_log(log);
        RequestOrResponse::Request(req)
    }

    async fn handle_response(&mut self, _ctx: &HttpContext, mut res: Response<Body>) -> Response<Body> {
        let body = res.body_mut();
        let s = body::to_bytes(body).await.unwrap();
        let res = Response::new(Body::from(s.clone()));
        let res_log = LogResponse::from(res);
        let mut history = &LogHistory::single().unwrap();
        history.set_resp(self.index, res_log);
        println!("{:?}", s);
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
    tracing_subscriber::fmt::init();
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
