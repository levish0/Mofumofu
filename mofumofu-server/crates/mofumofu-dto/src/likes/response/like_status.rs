use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct LikeStatusResponse {
    pub liked: bool,
}

impl IntoResponse for LikeStatusResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
