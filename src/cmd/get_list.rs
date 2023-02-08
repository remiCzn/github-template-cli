use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repo {
    pub name: String,
    pub path: String,
    pub url: String,
    pub download_url: Option<String>,
    pub r#type: String,
}

const URL: &str = "https://api.github.com/repos/remiCzn/templates/contents";

pub async fn get_list() -> Result<Vec<Repo>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client
        .get(URL)
        .header(USER_AGENT, "Anonymous")
        .send()
        .await?
        .text()
        .await?;
    let list: Vec<Repo> = serde_json::from_str(&body).unwrap();
    let mut result = vec![];
    for x in list {
        if x.r#type == "dir" {
            result.push(x);
        }
    }
    Ok(result)
}
