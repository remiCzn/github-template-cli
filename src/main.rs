extern crate core;

use crate::cmd::create::download_template;
use crate::cmd::get_list::get_list;
use std::env;

mod cmd;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    parse(args).await;
}

async fn parse(args: Vec<String>) {
    if let Some(arg) = &args.get(1) {
        match arg.as_str() {
            "get-list" => {
                let list = get_list()
                    .await
                    .expect("Error: Expected to get the template list ");
                println!("Template list:");
                for x in &list {
                    println!("- {}", x.name);
                }
            }
            "create" => {
                if let Some(name) = &args.get(2) {
                    download_template(name.as_str())
                        .await
                        .expect("Error: Expected to download template files");
                } else {
                    panic!("Argument needed for 'create' command")
                }
            }
            cmd => panic!("Unknown command: {}", cmd),
        };
    } else {
        panic!("Not enoug arguments");
    }
}
