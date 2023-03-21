use super::ApiResponseItem;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiResponse {
    pub total_count: i32,
    pub items: Vec<ApiResponseItem>,
}
