use std::{thread, time::Duration};

use burp_rs::{cmd::cmd::cmd, proxy::proxy::proxy, utils::log::init};

#[tokio::main]
async fn main() {
    init();
    thread::spawn(|| {
        cmd();
    });
    proxy("127.0.0.1:3000").await
    
}