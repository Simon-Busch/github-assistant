use reqwest::header::{HeaderValue, ACCEPT};
use reqwest::header;

pub async fn update_issue_status(repo_owner: String, repo_name: String, issue_number: i32, access_token: &String) -> Result<(), Box<dyn std::error::Error>> {
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
    let patch_url = format!("{}/repos/{}/{}/issues/{}", base_url, repo_owner, repo_name, issue_number);
    let response = client
        .patch(&patch_url)
        .header("Authorization", format!("token {}", access_token))
        .header("Content-Type", "application/json")
        .body(r#"{"state": "closed"}"#)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Issue closed successfully.");
    } else {
        println!("Failed to close issue.");
    }

    Ok(())
}
