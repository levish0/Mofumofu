use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct FollowStatusResponse {
    pub following: bool,
}

impl IntoResponse for FollowStatusResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
