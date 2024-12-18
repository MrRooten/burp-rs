#![allow(unused_parens)]

use std::thread;

use burp_rs::{
    cmd::cmd_inner::cmd,
    proxy::inner_proxy::proxy,
    scanner::scaner_thread,
    utils::{banner, config::{generate_key, get_config}, log::init},
};
use clap::{arg, Args, Parser, Subcommand};

async fn _main(addr: &str) {
    let _ = init();
    thread::spawn(|| {
        let _ = cmd();
    });
    proxy(addr).await
}

#[derive(Debug, Args)]
pub struct WebArgs {
    /// listen web address
    #[arg(short, long, default_value_t=("127.0.0.1:3000".to_string()))]
    listen: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Running web server
    Web(WebArgs),

    /// Generate ca
    GenerateKey,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    banner();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Web(web_args) => {
            let _ = get_config();
            let _ = scaner_thread();
            println!("listen on: {}", web_args.listen);
            _main(&web_args.listen).await;
        }
        Commands::GenerateKey => {
            generate_key();
        },
    }
}
