use super::{IssueComments, Label};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiResponseItem {
    #[serde(rename = "html_url")]
    pub url: String,
    pub title: String,
    pub number: i32,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub labels: Vec<Label>,
    pub body: Option<String>,
    pub repository: Option<String>,
    pub organization: Option<String>,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(skip_deserializing)]
    pub comments_list: Vec<IssueComments>,
    #[serde(skip_deserializing)]
    pub is_pr: bool,
}
