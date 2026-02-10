use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

use super::PostResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct PostListResponse {
    pub data: Vec<PostResponse>,
    pub has_newer: bool,
    pub has_older: bool,
}

impl IntoResponse for PostListResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
