use reqwest::header::{HeaderValue, ACCEPT};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use tokio;
use reqwest::header;
use std::error::Error;

fn init_variables() -> (String, String) {
  dotenv().ok();
  let username = std::env::var("GITHUB_USERNAME").expect("GITHUB_USERNAME must be set.");
  let access_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set.");
  return (username, access_token);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (username, access_token) = init_variables();
    println!("Username: {}", username);
    println!("Access Token: {}", access_token);

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
        "{}/search/issues?q=assignee:{}",
        base_url, username
    );
    let github_response = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;
    // println!("{:}", github_response);
    let json_response: ApiResponse = serde_json::from_str(&github_response)?;
    println!("{:?}", json_response.items);
    println!("{:?}", json_response.items[0].url);
    println!("{:?}", json_response.items[1].labels);

    Ok(())
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
  total_count: i32,
  items: Vec<ApiResponseItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiResponseItem {
  #[serde(rename = "html_url")]
  url: String,
  title: String,
  #[serde(skip)]
  number: i32,
  state: String,
  created_at: String,
  labels: Vec<Label>,
}
#[derive(Debug, Deserialize, Serialize)]
struct Label {
  name: String,
}
