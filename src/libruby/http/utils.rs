use std::str::FromStr;

use hyper::{body::Bytes, Method, Request, Body, Uri, Response, HeaderMap, header::{HeaderName}};
use rutie::{class, AnyObject, Array, Encoding, Hash, Integer, NilClass, Object, RString, methods, VM, AnyException, Exception};

use crate::{librs::http::utils::{HttpRequest, HttpResponse}, proxy::log::{ReqResLog, LogRequest, LogResponse}};

class!(RBHttpClient);

methods!(
    RBHttpClient,
    _rtself,
    fn send(request: Hash) -> AnyObject {
        let method_key = RString::from("method")
            .try_convert_to::<AnyObject>()
            .unwrap();
        let request = match request {
            Ok(o) => o,
            Err(e) => {
                let s = format!("{:?}",e);
                VM::raise_ex(AnyException::new("StandardError", Some(&s)));
                return NilClass::new().try_convert_to::<AnyObject>().unwrap();
            }
        };
        let method = request
            .at(&method_key)
            .try_convert_to::<RString>()
            .unwrap()
            .to_string();
        let send_request = ruby_hash_to_inner_request(&request);
        let mut response = None::<HttpResponse>;
        if method.eq_ignore_ascii_case("get") {
            let ret = HttpRequest::send(Method::GET, &send_request);
            let ret = match ret {
                Ok(o) => o,
                Err(e) => {
                    let s = format!("{:?}",e);
                    VM::raise_ex(AnyException::new("StandardError", Some(&s)));
                    return NilClass::new().try_convert_to::<AnyObject>().unwrap();
                }
            };

            response = Some(ret);
        } else if method.eq_ignore_ascii_case("post") {
            let ret = HttpRequest::send(Method::POST, &send_request);
            let ret = match ret {
                Ok(o) => o,
                Err(e) => {
                    let s = format!("{:?}",e);
                    VM::raise_ex(AnyException::new("StandardError", Some(&s)));
                    return NilClass::new().try_convert_to::<AnyObject>().unwrap();
                }
            };

            response = Some(ret);
        } else if method.eq_ignore_ascii_case("options") {
            let ret = HttpRequest::send(Method::OPTIONS, &send_request);
            let ret = match ret {
                Ok(o) => o,
                Err(e) => {
                    let s = format!("{:?}",e);
                    VM::raise_ex(AnyException::new("StandardError", Some(&s)));
                    return NilClass::new().try_convert_to::<AnyObject>().unwrap();
                }
            };

            response = Some(ret);
        }
        let response = match response {
            Some(o) => o,
            None => {
                VM::raise_ex(AnyException::new("StandardError", Some("Doesn't match response")));
                return NilClass::new().try_convert_to::<AnyObject>().unwrap();
            }
        };

        let mut ret = Hash::new();
        ret.store(
            RString::from("status_code"),
            Integer::from(response.get_status().as_u16() as u32),
        );
        let mut ruby_headers = Hash::new();
        let headers = response.get_headers();
        for kv in headers {
            let key_name = kv.0.as_str().to_string();
            let ruby_keyname = RString::from(key_name);
            if ruby_headers.at(&ruby_keyname).is_nil() {
                let mut ruby_value = Array::new();
                let value = kv.1.to_str().unwrap().to_string();
                ruby_value.push(RString::from(value));
                ruby_headers.store(ruby_keyname, ruby_value);
            } else {
                let mut ruby_value = ruby_headers
                    .at(&ruby_keyname)
                    .try_convert_to::<Array>()
                    .unwrap();
                let value = kv.1.to_str().unwrap().to_string();
                ruby_value.push(RString::from(value));
            }
        }

        ret.store(RString::from("headers"), ruby_headers);
        ret.store(
            RString::from("body"),
            RString::from_bytes(&response.get_body(), &Encoding::utf8()),
        );
        ret.store(RString::from("request"), request.clone());
        return ret.try_convert_to::<AnyObject>().unwrap();
    },
    
);

fn inner_request_to_ruby_hash(request: &HttpRequest) -> Hash {
    unimplemented!()
}

pub fn ruby_resp_hash_to_reqresplog(resp: &Hash) -> ReqResLog {
    let request = resp.at(&RString::from("request")).try_convert_to::<Hash>().unwrap();
    let headers = request.at(&RString::from("headers")).try_convert_to::<Hash>();
    let mut ori_headers = HeaderMap::new();
    match headers {
        Ok(o) => {
            let keys = o.each(|_key, _value| {
                let key = _key.try_convert_to::<RString>().unwrap().to_string();
                let value = _value.try_convert_to::<RString>().unwrap().to_string();
                let key = HeaderName::from_str(&key);
                ori_headers.insert(key.unwrap(), value.parse().unwrap());
            });
        },
        Err(e) => {
        }
    };
    let url = request.at(&RString::from("url")).try_convert_to::<RString>().unwrap().to_string();
    let body = request.at(&RString::from("body")).try_convert_to::<RString>();
    let body = match body {
        Ok(o) => o.to_string(),
        Err(e) => {
            "".to_string()
        }
    };
    let mut original = Request::new(Body::from(""));
    *original.uri_mut() = Uri::from_str(&url).unwrap();
    *original.headers_mut() = ori_headers;
    let req = LogRequest::from(original, Bytes::from(body));

    let body = resp.at(&RString::from("body")).try_convert_to::<RString>().unwrap().to_string();
    let mut original = Response::new(Body::from(""));
    let headers = resp.at(&RString::from("headers")).try_convert_to::<Hash>();
    let mut ori_headers = HeaderMap::new();
    match headers {
        Ok(o) => {
            let keys = o.each(|_key, _value| {
                let key = _key.try_convert_to::<RString>().unwrap().to_string();
                let value = _value.try_convert_to::<Array>().unwrap();
                for v in value {
                    let key = HeaderName::from_str(&key);
                    ori_headers.append(key.unwrap(), v.try_convert_to::<RString>().unwrap().to_string().parse().unwrap());
                }
                
            });
        },
        Err(e) => {
        }
    };
    *original.headers_mut() = ori_headers;
    let resp = LogResponse::from(original, Bytes::from(body));

    let mut log = ReqResLog::new(req);
    log.set_resp(resp);
    log
}

fn ruby_hash_to_inner_request(hash: &Hash) -> HttpRequest {
    let url_key = RString::from("url").try_convert_to::<AnyObject>().unwrap();
    let url = hash.at(&url_key).try_convert_to::<RString>().unwrap();
    let url = url.to_string();
    let mut req = HttpRequest::from_url(&url);
    let headers_key = RString::from("headers")
        .try_convert_to::<AnyObject>()
        .unwrap();
    let headers = hash.at(&headers_key).try_convert_to::<Hash>();
    if headers.is_ok() {
        let keys = headers.unwrap().each(|_key, value| {
            let key = _key.try_convert_to::<RString>().unwrap().to_string();
            let value = value.try_convert_to::<RString>().unwrap().to_string();
            req.set_header(&key, &value);
        });
    }

    let body_key = RString::from("body").try_convert_to::<AnyObject>().unwrap();
    let body = hash.at(&body_key).try_convert_to::<RString>();
    if body.is_ok() {
        let s = body.unwrap().to_bytes_unchecked().to_vec();
        let s = Bytes::from(s);
        req.set_body(s);
    }

    return req;
}

fn inner_response_to_ruby_hash(response: &HttpResponse) -> Hash {
    unimplemented!()
}

fn ruby_hash_to_inner_response(hash: Hash) -> HttpResponse {
    unimplemented!()
}

fn update_request_by_params(request: Hash, params: Array) -> AnyObject {
    let url = request.at(&RString::from("url"));
    let url = url.try_convert_to::<RString>().unwrap().to_string();
    let uri = Uri::from_str(&url).unwrap();
    let query = uri.query();
    unimplemented!()
}