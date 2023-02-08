use crate::cmd::get_list::{get_list, Repo};
use async_recursion::async_recursion;
use reqwest::header::USER_AGENT;
use std::error::Error;
use std::io::Cursor;
use std::path::Path;

pub async fn download_template(template_name: &str) -> Result<(), Box<dyn Error>> {
    let repos = get_list().await.expect("Expect to retrieve ");
    let repo = repos.iter().find(|&x| x.name == template_name);
    if let Some(ex_repo) = repo {
        println!("Exists: {:?}", ex_repo);
        download_folder(&ex_repo.url)
            .await
            .expect("Expect to download template");
    }
    Ok(())
}

async fn download_file(file_url: &String, filename: &String) -> Result<(), Box<dyn Error>> {
    println!("Download {filename}");
    let response = reqwest::Client::new()
        .get(file_url)
        .header(USER_AGENT, "Anonymous")
        .send()
        .await?;
    let path = Path::new(filename);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::File::create(filename)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    println!("{filename} downloaded");
    Ok(())
}

#[async_recursion]
async fn download_folder(folder_url: &String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(folder_url)
        .header(USER_AGENT, "Anonymous")
        .send()
        .await?
        .text()
        .await?;
    let list: Vec<Repo> = serde_json::from_str(&response).unwrap();
    for elt in &list {
        if elt.r#type == "dir" {
            download_folder(&elt.url)
                .await
                .expect("Can't download folder");
        } else if elt.r#type == "file" {
            if let Some(dl_url) = &elt.download_url {
                download_file(dl_url, &elt.path)
                    .await
                    .expect("Can't download file");
            } else {
                panic!("No download file");
            }
        }
    }
    Ok(())
}
