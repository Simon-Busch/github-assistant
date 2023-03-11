use reqwest::header;
use serde::Deserialize;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
struct Issue {
    // Define the fields you want to retrieve from the API response
}

#[derive(Debug, Deserialize)]
struct PullRequest {
    // Define the fields you want to retrieve from the API response
}

fn init_variables() {
  dotenv().ok();
  let username = std::env::var("GITHUB_USERNAME").expect("GITHUB_USERNAME must be set.");
  println!("The value of GITHUB_USERNAME is {}", username);
  let access_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set.");
  println!("The value of GITHUB_TOKEN is {}", access_token);
}

fn main()  {
    init_variables();
    // // Set the API endpoint to retrieve issues and pull requests associated with the specified username
    // let url = format!("https://api.github.com/search/issues?q=author:{}&type=issue,pr", username);

    // let client = reqwest::Client::new();

    // // Set the authorization header with the access token
    // let mut headers = header::HeaderMap::new();
    // headers.insert(header::AUTHORIZATION, format!("Bearer {}", access_token).parse()?);

    // // Make the API request and deserialize the response into the Issue and PullRequest structs
    // let response = client.get(&url).headers(headers).send()?.json::<ApiResponse>()?;
    // let issues = response.items.iter().filter_map(|item| match item {
    //     ApiResponseItem::Issue(issue) => Some(issue.clone()),
    //     ApiResponseItem::PullRequest(pr) => Some(pr.issue.clone()),
    // }).collect::<Vec<Issue>>();

    // println!("Issues: {:?}", issues);

    // Ok(())
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    items: Vec<ApiResponseItem>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ApiResponseItem {
    Issue(Issue),
    PullRequest(PullRequest),
}
