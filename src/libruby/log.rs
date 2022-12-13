use log::*;
use rutie::{class, methods, RString, AnyObject, NilClass, Object};
class!(RBLogger);

methods!(
    RBLogger,
    _rtself,
    fn debug(s: RString) -> AnyObject {
        let s = s.unwrap().to_string();
        let filename = "";
        debug!("{}:{}",filename, s);
        NilClass::new().try_convert_to::<AnyObject>().unwrap()
    },
    fn error(s: RString) -> AnyObject {
        let s = s.unwrap().to_string();
        let filename = "";
        error!("{}:{}", filename, s);
        NilClass::new().try_convert_to::<AnyObject>().unwrap()
    },
    fn info(s: RString) -> AnyObject {
        let s = s.unwrap().to_string();
        let filename = "";
        info!("{}:{}", filename, s);
        NilClass::new().try_convert_to::<AnyObject>().unwrap()
    },
    fn warn(s: RString) -> AnyObject {
        let s = s.unwrap().to_string();
        let filename = "";
        warn!("{}:{}", filename, s);
        NilClass::new().try_convert_to::<AnyObject>().unwrap()
    },
);
