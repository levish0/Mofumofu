use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

use super::draft_info::DraftInfo;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct GetDraftsResponse {
    pub drafts: Vec<DraftInfo>,
}

impl IntoResponse for GetDraftsResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
