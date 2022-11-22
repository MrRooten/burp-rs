use burp_rs::utils::log::init;
use burp_rs::{proxy::proxy::proxy};
use burp_rs::cmd::cmd::cmd;
use log::{debug, error, log_enabled, info, Level};
fn main() {
    //init();
    //cmd();
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(
        proxy("127.0.0.1:3000")
    )
}