use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

use super::ModerationLogResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct ModerationLogListResponse {
    pub data: Vec<ModerationLogResponse>,
    pub has_newer: bool,
    pub has_older: bool,
}

impl IntoResponse for ModerationLogListResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
