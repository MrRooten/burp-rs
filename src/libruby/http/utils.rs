use hyper::body::Bytes;
use rutie::{class, Hash, RString, Object, AnyObject, Array};

use crate::librs::http::utils::{HttpRequest, HttpResponse};

class!(RBHttpClient);

pub fn send(request: Hash) -> Hash {
    unimplemented!()
}

fn inner_request_to_ruby_hash(request: &HttpRequest) -> Hash {
    unimplemented!()
}

fn ruby_hash_to_inner_request(hash: Hash) -> HttpRequest {
    let url_key = RString::from("url").try_convert_to::<AnyObject>().unwrap();
    let url = hash.at(&url_key).try_convert_to::<RString>().unwrap();
    let url = url.to_string();
    let mut req = HttpRequest::from_url(&url);
    let headers_key = RString::from("headers").try_convert_to::<AnyObject>().unwrap();
    let headers = hash.at(&headers_key).try_convert_to::<Hash>().unwrap();
    let keys = headers.each(|_key, value| {
        let key = _key.try_convert_to::<RString>().unwrap().to_string();
        let value = value.try_convert_to::<RString>().unwrap().to_string();
        req.set_header(&key, &value);
    });

    let body_key = RString::from("body").try_convert_to::<AnyObject>().unwrap();
    let body = hash.at(&body_key).try_convert_to::<RString>().unwrap();
    let s = body.to_bytes_unchecked().to_vec();
    let s = Bytes::from(s);
    req.set_body(s);
    return req;
}

fn inner_response_to_ruby_hash(response: &HttpResponse) -> Hash {
    unimplemented!()
}

fn ruby_hash_to_inner_response(hash: Hash) -> HttpResponse {
    unimplemented!()
}