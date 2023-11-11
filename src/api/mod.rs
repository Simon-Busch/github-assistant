use crate::structs;
use structs::{ApiResponseItem};
use std::{error::Error};
mod fetch_github_data;
mod fetch_github_pr_review;
mod update_issue_status;
use fetch_github_data::get_github_response;
use fetch_github_pr_review::fetch_github_pr_review;
use chrono::{DateTime, NaiveDateTime, Utc};

fn parse_date_string(date_string: &str) -> DateTime<Utc> {
  let naive_date = NaiveDateTime::parse_from_str(date_string, "%Y-%m-%dT%H:%M:%SZ").unwrap();
  DateTime::from_naive_utc_and_offset(naive_date, Utc)
}

pub async fn init_gh_data(username: &str, access_token: &str) -> Result<(Vec<ApiResponseItem>, Vec<ApiResponseItem>, Vec<ApiResponseItem>, i32, i32, i32), Box<dyn Error>> {
  // Get list of open issues
  let issues_list_response_open = get_github_response(username, access_token, "open").await?;
  let mut issues_list_open = issues_list_response_open.items.to_owned();
  issues_list_open.sort_by_key(|i| parse_date_string(&i.updated_at));
  issues_list_open.reverse();
  // Get list of closed issues
  let issues_list_response_closed = get_github_response(username, access_token, "closed").await?;
  let mut issues_list_closed = issues_list_response_closed.items.to_owned();
  issues_list_closed.sort_by_key(|i| parse_date_string(&i.updated_at));
  issues_list_closed.reverse();
  // Get list of Assigned for review PR
  let assigned_pr = fetch_github_pr_review(username, access_token).await?;
  let mut assigned_pr_list = assigned_pr.items.to_owned();
  assigned_pr_list.sort_by_key(|i| parse_date_string(&i.updated_at));
  assigned_pr_list.reverse();

  // Convert the lengths of the objects lists to i32
  let issues_list_open_len = issues_list_response_open.total_count;
  let issues_list_closed_len = issues_list_response_closed.total_count;
  let assigned_pr_list_len = assigned_pr.total_count;

  Ok((issues_list_open, issues_list_closed, assigned_pr_list, issues_list_open_len, issues_list_closed_len, assigned_pr_list_len))
}

pub use update_issue_status::update_issue_status;
