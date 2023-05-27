use std::{
    env,
    thread::{self}, fs::File, io::Write,
};

use burp_rs::{
    cmd::cmd::cmd,
    proxy::{proxy::proxy},
    utils::{banner, log::init, config::{get_config}}, scanner::scaner_thread, librs::http::utils::{HttpRequest},
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
    let mut request = HttpRequest::from_url("http://baidu.com/?abc=123&asdjk=1232&badi=hello").unwrap();
    request.set_header("X-Forworded-For", "127.0.0.1:7890");
    request.set_header("cookie", "asdf=123;hello=456");
    let request = request.to_burp();
    let ss = request.get_params().unwrap();
    for s in ss {
        println!("{:?}",s);
    }
}

fn main() {
    
    banner();
    let _ = get_config();
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
