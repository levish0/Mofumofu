use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

use super::ReportResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct ReportListResponse {
    pub data: Vec<ReportResponse>,
    pub has_newer: bool,
    pub has_older: bool,
}

impl IntoResponse for ReportListResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
