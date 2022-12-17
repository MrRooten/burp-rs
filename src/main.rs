use std::{
    env,
    thread::{self, spawn},
    time::Duration,
};

use burp_rs::{
    cmd::cmd::cmd,
    librs::http::utils::HttpRequest,
    libruby::{rb_main::ruby_thread, utils::rb_init},
    proxy::{log::ReqResLog, proxy::proxy},
    utils::{banner, log::init},
};
use hyper::{Client, Method, Uri};
use rutie::rubysys::vm::ruby_init;
use tokio::runtime::Runtime;

#[tokio::main]
async fn _main(addr: &str) {
    let _ = init();
    thread::spawn(|| {
        let _ = cmd();
    });
    proxy(addr).await
}

#[tokio::main]
async fn test() {
    let client = Client::new();

    let ip_fut = async {
        let resp = client
            .get(Uri::from_static("http://127.0.0.1:8009/"))
            .await?;
        hyper::body::to_bytes(resp.into_body()).await
    };
    let headers_fut = async {
        let resp = client
            .get(Uri::from_static("http://127.0.0.1:8009/"))
            .await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    // Wait on both them at the same time:
    let (ip, headers) = futures::try_join!(ip_fut, headers_fut).unwrap();
}

fn main() {
    banner();
    let _ = ruby_thread();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        println!("{} 127.0.0.1:3000", args[0]);
        println!("{} default listen on: 127.0.0.1:3000", args[0]);
        _main("127.0.0.1:3000");
    } else if args[1].starts_with("test") {
        test();
    } else if args[1].starts_with("r_test") {
        let _ = rb_init();
        test();
        //let s = fs::read_to_string(args[2].to_string()).unwrap();
        //let _ = VM::require(&s);
    } else {
        println!("{} listen on: {}", args[0], args[1]);
        _main(&args[1]);
    }
}
