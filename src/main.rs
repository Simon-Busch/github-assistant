use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde::{Deserialize};
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
struct Issue {
    title: String,
    html_url: String,
}

#[derive(Debug, Deserialize)]
struct PullRequest {
    title: String,
    html_url: String,
}

#[derive(Debug, Deserialize)]
struct IssueOrPullRequest {
    html_url: String,
    #[serde(rename = "pull_request")]
    pull_request: Option<PullRequest>,
    #[serde(rename = "issue")]
    issue: Option<Issue>,
}


fn init_variables() -> (String, String) {
  dotenv().ok();
  let username = std::env::var("GITHUB_USERNAME").expect("GITHUB_USERNAME must be set.");
  let access_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set.");
  return (username, access_token);
}

#[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (username, accesss_token) = init_variables();
    println!("Username: {}", username);
    println!("Access Token: {}", accesss_token);
    let issues_and_pull_requests =
    get_issues_and_pull_requests_assigned_to_user(&username, &accesss_token).await?;
    println!("{:#?}", issues_and_pull_requests);
    Ok(())
}

// #[derive(Debug, Deserialize)]
// struct ApiResponse {
//     items: Vec<ApiResponseItem>,
// }

// #[derive(Debug, Deserialize)]
// #[serde(untagged)]
// enum ApiResponseItem {
//     Issue(Issue),
//     PullRequest(PullRequest),
// }

async fn get_issues_and_pull_requests_assigned_to_user(
  username: &str,
  access_token: &str,
) -> Result<Vec<IssueOrPullRequest>, reqwest::Error> {
  let base_url = "https://api.github.com";
  let url = format!(
      "{}/search/issues?q=assignee:{}",
      base_url, username
  );
  let client = reqwest::Client::new();
  let mut headers = HeaderMap::new();
  headers.insert(
      ACCEPT,
      HeaderValue::from_static("application/vnd.github.v3+json"),
  );
  headers.insert(
      "Authorization",
      HeaderValue::from_str(&format!("token {}", access_token)).unwrap(),
  );
  let response = client
      .get(&url)
      .headers(headers)
      .send()
      .await?
      .json::<serde_json::Value>()
      .await?;
  let issues_and_pull_requests: Vec<IssueOrPullRequest> = serde_json::from_value(
      response["items"].clone(),
  )
  .unwrap();
  Ok(issues_and_pull_requests)
}
