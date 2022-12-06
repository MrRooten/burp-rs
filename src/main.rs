use std::{thread, env};

use burp_rs::{cmd::{cmd::cmd}, proxy::proxy::proxy, utils::{log::init, banner}};



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
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        println!("{} 127.0.0.1:3000",args[0]);
        println!("{} default listen on: 127.0.0.1:3000", args[0]);
        _main("127.0.0.1:3000");
    } else {
        println!("{} listen on: {}", args[0], args[1]);
        _main(&args[1]);
    }
    
}