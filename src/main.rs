extern crate core;

use crate::cmd::get_list::get_list;
use std::env;

mod cmd;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    parse(args).await;
}

async fn parse(args: Vec<String>) {
    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "get-list" => get_list().await,
            cmd => panic!("Unknown command: {}", cmd),
        }
        .unwrap_or_else(|_| panic!("Can't execute command: {}", arg));
    } else {
        panic!("Not enoug arguments");
    }
}
