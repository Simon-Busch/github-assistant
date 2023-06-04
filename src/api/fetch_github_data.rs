use reqwest::header::{HeaderValue, ACCEPT};
use reqwest::header;

use crate::structs;
use structs::{ApiResponse, IssueComments};
use std::error::Error;


pub async fn get_github_response(username: &str, access_token: &str, status: &str) -> Result<ApiResponse, Box<dyn Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", access_token)).unwrap(),
    );
    headers.insert("User-Agent", HeaderValue::from_static("my app"));
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let base_url = "https://api.github.com";
    let url = format!(
        "{}/search/issues?q=assignee:{}+state:{}&per_page=100
        ",
        base_url, username, status
    );
    let github_response = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    let mut items: ApiResponse = serde_json::from_str(&github_response)?;
    for item in items.items.iter_mut() {
        let url_parts: Vec<&str> = item.url.split("/").collect();
        item.repository = Some(url_parts[url_parts.len() - 3].to_string());
        item.organization = Some(url_parts[url_parts.len() - 4].to_string());
        item.is_pr = url_parts.contains(&"pull");
        if item.state == "open" {
            let comments_url = &item.comments_url;
            let comments_response = client.get(comments_url).send().await?;
                if !comments_response.status().is_success() {
                    item.comments_list = vec![];
                } else {
                    let comments_json: Vec<IssueComments> = comments_response.json().await?;
                    item.comments_list = comments_json;
                }
        }
    }
    Ok(items)
}
