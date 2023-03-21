use crate::structs;
use structs::{ApiResponseItem};
use std::{error::Error};
mod fetch_github_data;
mod update_issue_status;
use fetch_github_data::get_github_response;


pub async fn init_gh_data(username: &str, access_token: &str) -> Result<(Vec<ApiResponseItem>, Vec<ApiResponseItem>, i32, i32), Box<dyn Error>> {
  // Get list of open issues
  let issues_list_response_open = get_github_response(username, access_token, "open").await?;
  let mut issues_list_open = issues_list_response_open.items.to_owned();
  issues_list_open.sort_by_key(|i| i.repository.clone().unwrap_or_default());

  // Get list of closed issues
  let issues_list_response_closed = get_github_response(username, access_token, "closed").await?;
  let mut issues_list_closed = issues_list_response_closed.items.to_owned();
  issues_list_closed.sort_by_key(|i| i.repository.clone().unwrap_or_default());

  // Convert the lengths of the issue lists to i32
  let issues_list_open_len = issues_list_response_open.total_count;
  let issues_list_closed_len = issues_list_response_closed.total_count;

  Ok((issues_list_open, issues_list_closed, issues_list_open_len, issues_list_closed_len))
}

pub use update_issue_status::update_issue_status;
