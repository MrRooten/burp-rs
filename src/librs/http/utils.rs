use std::str::FromStr;

extern crate hyper;
extern crate hyper_native_tls;

use hyper::{
    body::{self, Bytes},
    header::*,
    Body, Client, Method, Request, Response, Uri, StatusCode,
};

use crate::{
    proxy::log::{LogRequest, ReqResLog},
    utils::STError, st_error,
};

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

    pub fn clone(&self) -> HttpRequest {
        let mut request = Request::new(Body::from("")); 
        *request.uri_mut() = self.request.uri().clone();
        *request.headers_mut() = self.request.headers().clone();
        HttpRequest { request: request, body: self.body.clone() }
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

    pub fn clone_origial(&self) -> Request<Body> {
        let mut request = Request::new(Body::from("")); 
        *request.uri_mut() = self.request.uri().clone();
        *request.headers_mut() = self.request.headers().clone();
        request
    }

    pub fn get_body(&self) -> &Bytes {
        &self.body
    }
    
    pub fn from_log_request(request: &LogRequest) -> HttpRequest {
        unimplemented!()
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

    pub fn send(method: Method, request: &HttpRequest) -> Result<HttpResponse, STError> {
        let response = HttpRequest::send_async(Method::GET, &request);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let ret = rt.block_on(async {
            response.await
        });
        ret
    }

    pub async fn send_async(
        method: Method,
        request: &HttpRequest,
    ) -> Result<HttpResponse, STError> {
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
        if request.request.uri().to_string().starts_with("https") {
            if method == Method::GET {
                *req.method_mut() = Method::GET;
                let r = match clis.request(req).await {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(st_error!(e))
                    }
                };

                response = Some(r);
            } else if method == Method::POST {
                *req.method_mut() = Method::POST;
                let r = match clis.request(req).await {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(st_error!(e))
                    }
                };

                response = Some(r);
            } else if method == Method::PUT {
                *req.method_mut() = Method::PUT;
                let r = match clis.request(req).await {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(st_error!(e))
                    }
                };

                response = Some(r);
            } else if method == Method::OPTIONS {
                *req.method_mut() = Method::OPTIONS;
                let r = match clis.request(req).await {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(st_error!(e))
                    }
                };

                response = Some(r);
            }
        } else {
            let clis = Client::builder().build_http();
            if method == Method::GET {
                *req.method_mut() = Method::GET;
                let r = match clis.request(req).await {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(st_error!(e))
                    }
                };

                response = Some(r);
            } else if method == Method::POST {
                *req.method_mut() = Method::POST;
                let r = match clis.request(req).await {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(st_error!(e))
                    }
                };

                response = Some(r);
            } else if method == Method::PUT {
                *req.method_mut() = Method::PUT;
                let r = match clis.request(req).await {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(st_error!(e))
                    }
                };

                response = Some(r);
            } else if method == Method::OPTIONS {
                *req.method_mut() = Method::OPTIONS;
                let r = match clis.request(req).await {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(st_error!(e))
                    }
                };

                response = Some(r);
            }
        }
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

        Ok(HttpResponse::from(request,response, body))
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    req : HttpRequest,
    resp: Response<Body>,
    body: Bytes,
}

impl HttpResponse {
    pub fn from(req: &HttpRequest, resp: Response<Body>, body: Bytes) -> Self {
        Self {
            req : req.clone(),
            resp: resp,
            body: body,
        }
    }

    pub fn get_status(&self) -> StatusCode {
        self.resp.status()
    }

    pub fn get_header(&self, key: &str) -> String {
        let c_type = self.resp.headers().get(key);
        match c_type {
            Some(s) => {
                return s.to_str().unwrap().to_string();
            },
            None => {
                return "".to_string();
            }
        }
    }

    pub fn get_headers(&self) -> &HeaderMap {
        self.resp.headers()
    }

    pub fn get_body(&self) -> &Bytes {
        &self.body
    }

    pub fn get_httplog(&self) -> ReqResLog {
        unimplemented!()
    }

    pub fn get_request(&self) -> &HttpRequest {
        &self.req
    }

    pub fn clone_original(&self) -> Response<Body> {
        let mut response = Response::new(Body::from(""));
        *response.headers_mut() = self.resp.headers().clone();
        *response.status_mut() = self.resp.status().clone();
        response
    }

}
