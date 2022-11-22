use hyper::{body::Bytes, HeaderMap};

use crate::proxy::log::ReqResLog;

pub struct HttpSession {

}

impl HttpRequest {
    pub fn post(request: &HttpRequest) {

    }

    pub fn get(request: &HttpRequest) {

    }


}

pub struct HttpRequest {
    headers     : HeaderMap,
    url         : String,
    version     : String,

}

impl HttpRequest {
    pub fn from_url(url: &str) -> HttpRequest {
        unimplemented!()
    }

    pub fn set_header(key: String, value: String) {
        unimplemented!()
    }

    pub fn set_body(body: Bytes) {
        unimplemented!()
    }

}

pub struct HttpResponse {

}

impl HttpResponse {
    pub fn get_httplog(&self) -> ReqResLog {
        unimplemented!()
    }
    
    pub fn get_request(&self) -> HttpRequest {
        unimplemented!()
    }
}