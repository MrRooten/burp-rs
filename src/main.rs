use std::{thread, time::Duration};

use burp_rs::{cmd::cmd::cmd, proxy::proxy::proxy};

#[tokio::main]
async fn main() {
    thread::spawn(|| {
        cmd();
    });
    proxy("127.0.0.1:3000").await
    
}