use std::{thread, env};

use burp_rs::{utils::{banner, log::init}, libruby::utils::rb_init, modules::{active::ruby_scan::RBModule, IActive}, proxy::proxy::proxy, cmd::cmd::cmd};
use colored::Colorize;
use rutie::{VM, eval, rubysys::encoding::rb_locale_encindex};


pub fn test()  {
    rb_init();
    eval!("require 'json'");
    let a = RBModule::new("./test.rb").unwrap();
    a.metadata();
}

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