use super::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IssueComments {
    pub body: String,
    pub user: User,
}
