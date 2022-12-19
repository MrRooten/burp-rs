use std::{
    env,
    thread::{self},
};

use burp_rs::{
    cmd::cmd::cmd,
    proxy::{proxy::proxy},
    utils::{banner, log::init, config::Config}, scanner::scaner_thread,
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
    let config = Config::read().unwrap();
    println!("{:?}",config.get("http.parallel_per_site"));
}

fn main() {
    banner();
    let _ = scaner_thread();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        println!("{} 127.0.0.1:3000", args[0]);
        println!("{} default listen on: 127.0.0.1:3000", args[0]);
        _main("127.0.0.1:3000");
    } else if args[1].starts_with("test") {
        test();
    } else if args[1].starts_with("r_test") {
        //let s = fs::read_to_string(args[2].to_string()).unwrap();
        //let _ = VM::require(&s);
    } else {
        println!("{} listen on: {}", args[0], args[1]);
        _main(&args[1]);
    }
}
