use burp_rs::proxy::proxy::proxy;

fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(
        proxy("127.0.0.1:3000")
    )
}