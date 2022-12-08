use std::{
    env,
    thread::{self, spawn},
};

use burp_rs::{
    cmd::cmd::cmd,
    libruby::{utils::{object_to_string, rb_init}, rb_main::{self, ruby_thread}},
    modules::{active::ruby_scan::RBModule, IActive},
    proxy::proxy::proxy,
    utils::{banner, log::init},
};
use colored::Colorize;
use rutie::{eval, rubysys::encoding::rb_locale_encindex, Fixnum, Object, Thread, VM};

pub fn test() {
    let b = ruby_thread();
    b.join();
}
#[tokio::main]
async fn _main(addr: &str) {
    let _ = init();
    thread::spawn(|| {
        let _ = cmd();
    });
    proxy(addr).await
}

fn main() {
    banner();
    test();
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
