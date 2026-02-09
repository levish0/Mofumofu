use crate::dto::comment::response::CommentInfo;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct GetCommentsResponse {
    pub comments: Vec<CommentInfo>,
    pub total_count: u64,
    pub page: u32,
    pub per_page: u32,
    pub has_next: bool,
}

#[derive(Serialize, ToSchema)]
pub struct GetRepliesResponse {
    pub replies: Vec<CommentInfo>,
    pub total_count: u64,
    pub page: u32,
    pub per_page: u32,
    pub has_next: bool,
}

impl IntoResponse for GetCommentsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl IntoResponse for GetRepliesResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
