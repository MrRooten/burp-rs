use std::str::FromStr;

extern crate hyper;
extern crate hyper_native_tls;

use hyper::{
    body::{self, Bytes},
    header::*,
    Body, Client, Method, Request, Response, Uri,
};
use hyper_native_tls::NativeTlsClient;
use hyper_rustls::HttpsConnector;

use crate::{proxy::log::ReqResLog, utils::STError};

pub struct HttpSession {}

impl HttpRequest {
    pub fn post(request: &HttpRequest) {}

    pub fn get(request: &HttpRequest) {}
}

#[derive(Debug)]
pub struct HttpRequest {
    request: Request<Body>,
    body: Bytes,
}

impl HttpRequest {
    pub fn from_bytes(bs: Bytes) -> HttpRequest {
        unimplemented!()
    }

    pub fn to_bytes(&self) -> Bytes {
        unimplemented!()
    }

    pub fn from_url(url: &str) -> HttpRequest {
        let mut request = Request::new(Body::from(""));
        *request.uri_mut() = Uri::from_str(url).unwrap();
        HttpRequest {
            request: request,
            body: Bytes::new(),
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.request.headers_mut().append(
            HeaderName::from_str(key).unwrap(),
            HeaderValue::from_str(value).unwrap(),
        );
    }

    pub fn set_body(&mut self, body: Bytes) {
        self.body = body.clone();
    }

    fn set_method(&mut self, method: Method) {
        *self.request.method_mut() = method;
    }

    pub async fn send(method: Method, request: &HttpRequest) -> Result<HttpResponse, STError> {
        let cli = Client::new();

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http1()
            .build();

        let clis: Client<_, hyper::Body> = Client::builder().build(https);

        let mut response = None::<Response<Body>>;
        let mut req = Request::builder()
            .uri(request.request.uri())
            .body(Body::from(""))
            .expect("");
        req.extensions().clone_from(&request.request.extensions());
        *req.headers_mut() = request.request.headers().clone();
        *req.version_mut() = request.request.version();
        req.uri_mut().clone_from(request.request.uri());
        *req.body_mut() = Body::from(request.body.clone());
        println!("{:?}", req);
        if request.request.uri().to_string().starts_with("https") {
            if method == Method::GET {
                *req.method_mut() = Method::GET;
                response = Some(clis.request(req).await.unwrap());
            } else if method == Method::POST {
                *req.method_mut() = Method::POST;
                response = Some(clis.request(req).await.unwrap());
            } else if method == Method::PUT {
                *req.method_mut() = Method::PUT;
                response = Some(clis.request(req).await.unwrap());
            } else if method == Method::OPTIONS {
                *req.method_mut() = Method::OPTIONS;
                response = Some(clis.request(req).await.unwrap());
            }
        } else {
            if method == Method::GET {
                *req.method_mut() = Method::GET;
                response = Some(cli.request(req).await.unwrap());
            } else if method == Method::POST {
                *req.method_mut() = Method::POST;
                response = Some(cli.request(req).await.unwrap());
            } else if method == Method::PUT {
                *req.method_mut() = Method::PUT;
                response = Some(cli.request(req).await.unwrap());
            } else if method == Method::OPTIONS {
                *req.method_mut() = Method::OPTIONS;
                response = Some(cli.request(req).await.unwrap());
            }
        }
        println!("{:?}", response);
        let mut response = match response {
            Some(res) => res,
            None => {
                return Err(STError::new("Error get response from request"));
            }
        };
        let body = match body::to_bytes(response.body_mut()).await {
            Ok(o) => o,
            Err(e) => Bytes::new(),
        };

        Ok(HttpResponse::from(response, body))
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    resp: Response<Body>,
    body: Bytes,
}

impl HttpResponse {
    pub fn from(resp: Response<Body>, body: Bytes) -> Self {
        Self {
            resp: resp,
            body: body,
        }
    }

    pub fn get_httplog(&self) -> ReqResLog {
        unimplemented!()
    }

    pub fn get_request(&self) -> HttpRequest {
        unimplemented!()
    }
}
