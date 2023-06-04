use super::ApiResponseItem;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub total_count: i32,
    pub items: Vec<ApiResponseItem>,
}
