use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

use super::CommentResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct CommentListResponse {
    pub data: Vec<CommentResponse>,
    pub has_newer: bool,
    pub has_older: bool,
}

impl IntoResponse for CommentListResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
