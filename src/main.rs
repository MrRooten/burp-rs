use std::{
    env,
    thread::{self}, 
};

use burp_rs::{
    cmd::cmd::cmd,
    libruby::{rb_main::{ruby_thread}, utils::rb_init},
    proxy::{proxy::proxy, log::ReqResLog},
    utils::{banner, log::init}, librs::http::utils::HttpRequest,
};
use hyper::Method;

#[tokio::main]
async fn _main(addr: &str) {
    let _ = init();
    thread::spawn(|| {
        let _ = cmd();
    });
    proxy(addr).await
}

fn test() {
    let request = HttpRequest::from_url("http://cn.bing.com");
    let resp = HttpRequest::send(Method::GET, &request);
    let log = ReqResLog::from_http_response(&resp.unwrap());
    println!("{:?}",log);
}

fn main() {
    banner();
    let _ = ruby_thread();
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    if args.len() < 2 {
        println!("{} 127.0.0.1:3000", args[0]);
        println!("{} default listen on: 127.0.0.1:3000", args[0]);
        _main("127.0.0.1:3000");
    } else if args[1].starts_with("test") {
        
    } else if args[1].starts_with("r_test") {
        let _ = rb_init();
        test();
        //let s = fs::read_to_string(args[2].to_string()).unwrap();
        //let _ = VM::require(&s);
    }
    else {
        println!("{} listen on: {}", args[0], args[1]);
        _main(&args[1]);
    }
}
