use crate::dto::post::response::PostListItem;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct UserPostsResponse {
    pub posts: Vec<PostListItem>,
}

impl IntoResponse for UserPostsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
