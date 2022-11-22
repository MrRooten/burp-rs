
use burp_rs::librs::http::utils::HttpRequest;
use burp_rs::{proxy::proxy::proxy};
use hyper::Method;
use log::{debug, error, log_enabled, info, Level};

async fn test() {
    let mut request = HttpRequest::from_url("http://127.0.0.1:8888");
    request.set_header("abc", "345");
    HttpRequest::send(Method::GET, &request).await;
}
fn main() {
    //init();
    //cmd();

    
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(
        //proxy("127.0.0.1:3000")
        test()
    )
}