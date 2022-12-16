use hyper::Version;
use rutie::{
    class, methods, AnyObject, Array, Encoding, Fixnum, Hash, NilClass, Object, RString,
};
use serde_json::Value;

use crate::{proxy::log::{LogHistory, ParamType, RequestParam}, libruby::log::error};

class!(RBReqResLog);

class!(RBHttpLog);

methods!(
    RBHttpLog,
    _rtself,
    fn get_http_req(i: Fixnum) -> AnyObject {
        _get_http_req(i.unwrap())
    },
    fn get_http_resp(i: Fixnum) -> AnyObject {
        let index = i.unwrap().to_u32();
        let reqresp = LogHistory::get_httplog(index);
        let reqresp = match reqresp {
            Some(s) => s,
            None => {
                return NilClass::new().try_convert_to::<AnyObject>().unwrap();
            }
        };

        let mut resp_hash = Hash::new();
        let response = match reqresp.get_response() {
            Some(s) => s,
            None => {
                return NilClass::new().try_convert_to::<AnyObject>().unwrap();
            }
        };

        let mut ruby_headers = Hash::new();
        for kv in response.get_headers() {
            let key_name = kv.0.as_str();
            let ruby_keyname = RString::from(key_name);
            if ruby_headers.at(&ruby_keyname).is_nil() {
                let mut ruby_value = Array::new();
                let value = kv.1.to_str().unwrap();
                ruby_value.push(RString::from(value));
                ruby_headers.store(ruby_keyname, ruby_value);
            } else {
                let mut ruby_value = ruby_headers
                    .at(&ruby_keyname)
                    .try_convert_to::<Array>()
                    .unwrap();
                let value = kv.1.to_str().unwrap();
                ruby_value.push(RString::from(value));
            }
        }

        resp_hash.store(RString::from("headers"), ruby_headers);
        resp_hash.store(
            RString::from("body"),
            RString::from_bytes(response.get_body(), &Encoding::utf8()),
        );
        resp_hash.try_convert_to::<AnyObject>().unwrap()
    },
);

pub fn req_params_to_ruby_params(params: &Vec<RequestParam>) -> AnyObject {
    let mut result = Array::new();
    for param in params {
        let mut item = Array::new();
        if param.get_param_type().eq(&ParamType::Get) {
            item.push(RString::from("get"));
            item.push(RString::from(param.get_key().to_string()));
            item.push(RString::from(param.get_value().to_string()));
        } else if param.get_param_type().eq(&ParamType::Cookie) {
            item.push(RString::from("cookie"));
            item.push(RString::from(param.get_key().to_string()));
            item.push(RString::from(param.get_value().to_string()));
        } else if param.get_param_type().eq(&ParamType::GetRaw) {
            item.push(RString::from("get_raw"));
            item.push(RString::from(param.get_key().to_string()));
            item.push(RString::from(param.get_value().to_string()));
        } else if param.get_param_type().eq(&ParamType::Header) {
            item.push(RString::from("header"));
            item.push(RString::from(param.get_key().to_string()));
            item.push(RString::from(param.get_value().to_string()));
        } else if param.get_param_type().eq(&ParamType::Post) {
            item.push(RString::from("post"));
            item.push(RString::from(param.get_key().to_string()));
            item.push(RString::from(param.get_value().to_string()));
        } else if param.get_param_type().eq(&ParamType::PostRaw) {
            item.push(RString::from("post_raw"));
            item.push(RString::from(param.get_key().to_string()));
            item.push(RString::from(param.get_value().to_string()));
        } else if param.get_param_type().eq(&ParamType::Json) {
            item.push(RString::from("json"));
            item.push(RString::from(param.get_json()));
        }

        result.push(item);
    }
    return result.try_convert_to::<AnyObject>().unwrap();
}

pub fn ruby_params_to_req_params(params: &Array) -> Vec<RequestParam> {
    let mut result = vec![];

    for i in 0..params.length() {
        let value = params.at(i as i64);
        let item = value.try_convert_to::<Array>().unwrap();
        let v_type = item.at(0).try_convert_to::<RString>().unwrap().to_string();
        if v_type.eq("get") {
            let key = item.at(1).try_convert_to::<RString>().unwrap().to_string();
            let value = item.at(2).try_convert_to::<RString>().unwrap().to_string();
            let param = RequestParam::new(ParamType::Get, &key, &value);
            result.push(param);
        } else if v_type.eq("cookie") {
            let key = item.at(1).try_convert_to::<RString>().unwrap().to_string();
            let value = item.at(2).try_convert_to::<RString>().unwrap().to_string();
            let param = RequestParam::new(ParamType::Cookie, &key, &value);
            result.push(param);
        } else if v_type.eq("post") {
            let key = item.at(1).try_convert_to::<RString>().unwrap().to_string();
            let value = item.at(2).try_convert_to::<RString>().unwrap().to_string();
            let param = RequestParam::new(ParamType::Post, &key, &value);
            result.push(param);
        } else if v_type.eq("post_raw") {
            let key = item.at(1).try_convert_to::<RString>().unwrap().to_string();
            let value = item.at(2).try_convert_to::<RString>().unwrap().to_string();
            let param = RequestParam::new(ParamType::PostRaw, &key, &value);
            result.push(param);
        } else if v_type.eq("header") {
            let key = item.at(1).try_convert_to::<RString>().unwrap().to_string();
            let value = item.at(2).try_convert_to::<RString>().unwrap().to_string();
            let param = RequestParam::new(ParamType::Header, &key, &value);
            result.push(param);
        } else if v_type.eq("get_raw") {
            let key = item.at(1).try_convert_to::<RString>().unwrap().to_string();
            let value = item.at(2).try_convert_to::<RString>().unwrap().to_string();
            let param = RequestParam::new(ParamType::GetRaw, &key, &value);
            result.push(param);
        } else if v_type.eq("json") {
            let json = item.at(1).try_convert_to::<RString>().unwrap().to_string();
            let v: Value = serde_json::from_str(&json).unwrap();
            let param = RequestParam::from_json(v);
        }
    }
    result
}

fn _get_http_req(i: Fixnum) -> AnyObject {
    let index = i.to_u32();
    let reqresp = LogHistory::get_httplog(index);
    let reqresp = match reqresp {
        Some(s) => s,
        None => {
            return NilClass::new().try_convert_to::<AnyObject>().unwrap();
        }
    };

    let mut req_hash = Hash::new();
    let request = match reqresp.get_request() {
        Some(s) => s,
        None => {
            return NilClass::new().try_convert_to::<AnyObject>().unwrap();
        }
    };
    let method = request.get_method();
    req_hash.store(RString::from("method"), RString::from(method));
    let version = request.get_proto();
    let s: String;
    if version.eq(&Version::HTTP_09) {
        s = "http_09".to_string();
    } else if version.eq(&Version::HTTP_10) {
        s = "http_10".to_string();
    } else if version.eq(&Version::HTTP_11) {
        s = "http_11".to_string();
    } else if version.eq(&Version::HTTP_2) {
        s = "http_2".to_string();
    } else if version.eq(&Version::HTTP_3) {
        s = "http_3".to_string();
    } else {
        s = "http_11".to_string();
    }
    req_hash.store(RString::from("proto"), RString::from(s));
    let url = request.get_url();
    let url = RString::from(url);
    let url_key = RString::from("url");
    req_hash.store(url_key, url);
    let mut ruby_headers = Hash::new();
    for kv in request.get_headers() {
        let key_name = kv.0.as_str();
        let ruby_keyname = RString::from(key_name);
        if ruby_headers.at(&ruby_keyname).is_nil() {
            let mut ruby_value = Array::new();
            let value = match kv.1.to_str() {
                Ok(o) => o,
                Err(e) => {
                    log::error!("{}",e);
                    continue;
                }
            };
            ruby_value.push(RString::from(value));
            ruby_headers.store(ruby_keyname, ruby_value);
        } else {
            let mut ruby_value = ruby_headers
                .at(&ruby_keyname)
                .try_convert_to::<Array>()
                .unwrap();
            let value = kv.1.to_str().unwrap();
            ruby_value.push(RString::from(value));
        }
    }

    req_hash.store(RString::from("headers"), ruby_headers);
    req_hash.store(
        RString::from("body"),
        RString::from_bytes(request.get_body(), &Encoding::utf8()),
    );
    req_hash.try_convert_to::<AnyObject>().unwrap()
}
