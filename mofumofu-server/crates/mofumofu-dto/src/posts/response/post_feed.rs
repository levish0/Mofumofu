use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

use super::PostResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct PostFeedResponse {
    pub data: Vec<PostResponse>,
    pub page: u32,
    pub page_size: u32,
    pub has_more: bool,
    pub total_count: u64,
}

impl IntoResponse for PostFeedResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
