use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct FollowStatusResponse {
    pub is_following: bool,
}

impl IntoResponse for FollowStatusResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
