use reqwest::header::{HeaderValue, ACCEPT};
use reqwest::header;

use crate::structs;
use structs::ApiResponse;
use std::error::Error;

pub async fn fetch_github_pr_review(username: &str, access_token: &str) -> Result<ApiResponse, Box<dyn Error>> {
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
      "{}/search/issues?q=type:pr+review-requested:{}+state:open
      ",
      base_url, username
  );
  let github_response = client
      .get(url)
      .send()
      .await?
      .text()
      .await?;

    let items: ApiResponse = serde_json::from_str(&github_response)?;
    Ok(items)
}
