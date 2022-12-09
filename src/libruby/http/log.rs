use rutie::{class, methods, Fixnum, AnyObject, NilClass, Object, Hash, RString, Array, Encoding};

use crate::proxy::log::LogHistory;

class!(RBReqResLog);


class!(RBHttpLog);

methods!(
    RBHttpLog,
    _rtself,
    fn get_http_req(i: Fixnum) -> AnyObject {
        let index = i.unwrap().to_u32();
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
                let value = kv.1.to_str().unwrap();
                ruby_value.push(RString::from(value));
                ruby_headers.store(ruby_keyname, ruby_value);
            } else {
                let mut ruby_value = ruby_headers.at(&ruby_keyname).try_convert_to::<Array>().unwrap();
                let value = kv.1.to_str().unwrap();
                ruby_value.push(RString::from(value));
            }
        }
    
        req_hash.store(RString::from("header"), ruby_headers);
        req_hash.store(RString::from("body"), RString::from_bytes(request.get_body(), &Encoding::utf8()));
        req_hash.try_convert_to::<AnyObject>().unwrap()
    }
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
                    let mut ruby_value = ruby_headers.at(&ruby_keyname).try_convert_to::<Array>().unwrap();
                    let value = kv.1.to_str().unwrap();
                    ruby_value.push(RString::from(value));
                }
            }
        
            resp_hash.store(RString::from("header"), ruby_headers);
            resp_hash.store(RString::from("body"), RString::from_bytes(response.get_body(),&Encoding::utf8()));
            resp_hash.try_convert_to::<AnyObject>().unwrap()
    }
);


