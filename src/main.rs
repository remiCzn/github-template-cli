extern crate core;

use clap::Parser;

use crate::cmd::create::download_template;
use crate::cmd::get_list::get_list;

mod cmd;

#[derive(Parser, Debug)]
#[command(author = "remiCzn", version = "0.0.1")]
pub enum Args {
    GetList,
    Pull { template_name: String, folder_name: String },
}

#[tokio::main]
async fn main() {
    // let args: Vec<String> = env::args().collect();
    // parse(args).await;
    let args = Args::parse();
    match args {
        Args::GetList => {
            let list = get_list()
                .await
                .expect("Error: Expected to get the template list ");
            println!("Template list:");
            for x in &list {
                println!("- {}", x.name);
            }
        }
        Args::Pull { template_name, folder_name: _ } => {
            download_template(template_name.as_str())
                .await
                .expect("Error: Expected to download template files");
        }
    }
}
