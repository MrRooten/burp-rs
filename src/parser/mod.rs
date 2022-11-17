pub enum Protocol {
    HTTPS,
    HTTP
}

pub enum Method {
    GET,
    POST,
    OPTIONS,
    PUT,
    DELETE
}
pub struct Request {
    time        : String,
    url         : String,
    host_ip     : String,
    port        : u16,
    protocol    : Protocol,
    method      : Method,
    path        : String,
    raw         : Vec<u8>
}

pub struct Response {
    status      : u16,
    length      : u32,
    mimetype    : String,
    raw         : Vec<u8>
}

pub struct BurpHttpPair {

}