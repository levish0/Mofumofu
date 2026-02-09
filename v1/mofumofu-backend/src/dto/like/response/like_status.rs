use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LikeStatusResponse {
    pub is_liked: bool,
}

impl IntoResponse for LikeStatusResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
