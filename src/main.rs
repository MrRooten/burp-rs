use std::{
    env,
    thread::{self}, fs,
};

use burp_rs::{
    cmd::cmd::cmd,
    libruby::{rb_main::{ruby_thread}, utils::rb_init},
    proxy::proxy::proxy,
    utils::{banner, log::init}, librs::http::utils::HttpRequest,
};
use hyper::Method;
use rutie::eval;

#[tokio::main]
async fn _main(addr: &str) {
    let _ = init();
    thread::spawn(|| {
        let _ = cmd();
    });
    proxy(addr).await
}

fn test() {
    let request = HttpRequest::from_url("https://cn.bing.com");
    let _ = HttpRequest::send(Method::GET, &request);
    
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
        test();
    } else if args[1].starts_with("r_test") {
        let _ = rb_init();
        let s = fs::read_to_string(args[2].to_string()).unwrap();
        let _ = eval!(&s);
    }
    else {
        println!("{} listen on: {}", args[0], args[1]);
        _main(&args[1]);
    }
}
