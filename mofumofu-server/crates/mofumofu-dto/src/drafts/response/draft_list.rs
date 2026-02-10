use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

use super::DraftResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct DraftListResponse {
    pub data: Vec<DraftResponse>,
}

impl IntoResponse for DraftListResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
