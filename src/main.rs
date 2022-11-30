use std::{thread};

use burp_rs::{cmd::{cmd::cmd}, proxy::proxy::proxy, utils::log::init};

#[tokio::main]
async fn _main() {
    let _ = init();
    thread::spawn(|| {
        let _ = cmd();
    });
    proxy("127.0.0.1:3000").await

}

fn main() {
    _main();
}