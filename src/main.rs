use burp_rs::proxy::proxy::proxy;
use burp_rs::cmd::cmd::cmd;
fn main() {
    //cmd();
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(
        proxy("127.0.0.1:3000")
    )
}