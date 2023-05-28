use std::{
    env,
    thread::{self}, fs::File, io::Write, sync::Arc, time,
};

use burp_rs::{
    cmd::cmd::cmd,
    proxy::{proxy::proxy},
    utils::{banner, log::init, config::{get_config}}, scanner::scaner_thread, librs::http::utils::{HttpRequest},
};
use tokio::sync::Semaphore;



async fn _main(addr: &str) {
    let _ = init();
    thread::spawn(|| {
        let _ = cmd();
    });
    proxy(addr).await
}


async fn test() {
    let sem = Arc::new(Semaphore::new(3));
    let mut v = vec![];
    for i in 1..100 {
        let _s = sem.clone();
        let h = tokio::spawn(async move  {
            let s = _s.acquire().await;
            println!("i:{}", i);
            thread::sleep(time::Duration::from_millis(1000));
        });
        v.push(h);
    }

    for i in v {
        i.await;
    }
}

#[tokio::main]
async fn main() {
    banner();
    let _ = get_config();
    let _ = scaner_thread();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        println!("{} 127.0.0.1:3000", args[0]);
        println!("{} default listen on: 127.0.0.1:3000", args[0]);
        _main("127.0.0.1:3000").await;
    } else if args[1].starts_with("test") {
        test().await;
    } else if args[1].starts_with("r_test") {
        //let s = fs::read_to_string(args[2].to_string()).unwrap();
        //let _ = VM::require(&s);
    } else {
        println!("{} listen on: {}", args[0], args[1]);
        _main(&args[1]).await;
    }
    
}
