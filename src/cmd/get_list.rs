use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Repo {
    name: String,
    path: String,
    url: String,
    r#type: String,
}

const URL: &str = "https://api.github.com/repos/remiCzn/templates/contents";

pub async fn get_list() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(URL)
        .header(USER_AGENT, "Anonymous")
        .send()
        .await?
        .text()
        .await?;
    let list: Vec<Repo> = serde_json::from_str(&body).unwrap();
    let a: Vec<String> = list
        .iter()
        .filter(|&x| x.r#type == "dir")
        .map(|x| x.clone().name)
        .collect();
    println!("Template list:");
    for x in &a {
        println!("  - {}", x);
    }
    Ok(a)
}
