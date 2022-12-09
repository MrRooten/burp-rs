use std::{
    env,
    thread::{self},
};

use burp_rs::{
    cmd::cmd::cmd,
    libruby::{rb_main::{ruby_thread}},
    proxy::proxy::proxy,
    utils::{banner, log::init},
};

#[tokio::main]
async fn _main(addr: &str) {
    let _ = init();
    thread::spawn(|| {
        let _ = cmd();
    });
    proxy(addr).await
}

fn test() {
    println!("hello");
}

fn main() {
    banner();
    let _ = ruby_thread();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        println!("{} 127.0.0.1:3000", args[0]);
        println!("{} default listen on: 127.0.0.1:3000", args[0]);
        _main("127.0.0.1:3000");
    } else if args[1].eq("test") {
        test();
    } else {
        println!("{} listen on: {}", args[0], args[1]);
        _main(&args[1]);
    }
}
