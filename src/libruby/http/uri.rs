use std::str::FromStr;

use hyper::Uri;
use hyper::http::uri::Scheme;
use rutie::AnyException;
use rutie::AnyObject;
use rutie::Exception;
use rutie::Integer;
use rutie::NilClass;
use rutie::Object;
use rutie::RString;
use rutie::Hash;
use rutie::VM;
use rutie::class;
use rutie::methods;
class!(RBUriParser);

methods!(
    RBUriParser,
    _rtself,
    fn parse(uri: RString) -> AnyObject {
        _parse(uri.unwrap())
    }
);

fn _parse(uri: RString) -> AnyObject {
    let parse = match Uri::from_str(uri.to_str()) {
        Ok(o) => {
            o
        },
        Err(e) => {
            let s = format!("{:?}",e);
            VM::raise_ex(AnyException::new("StandardError", Some(&s)));
            return NilClass::new().try_convert_to::<AnyObject>().unwrap();
        }
    };

    let mut result = Hash::new();
    let host = match parse.host() {
        Some(s) => s.to_string(),
        None => "".to_string()
    };
    result.store(RString::from("host"), RString::from(host));
    result.store(RString::from("path"), RString::from(parse.path().to_string()));
    result.store(RString::from("query"), RString::from(parse.query().unwrap_or("").to_string()));
    result.store(RString::from("scheme"), RString::from(parse.scheme_str().unwrap_or("http").to_string()));
    let port = match parse.port() {
        Some(s) => s.as_u16(),
        None => {
            if parse.scheme().is_none() {
                80
            } else if parse.scheme().unwrap().eq(&Scheme::HTTPS) {
                443
            } else {
                80
            }
        }
    };
    result.store(RString::from("port"), Integer::from(port as u32));
    result.try_convert_to::<AnyObject>().unwrap()
}