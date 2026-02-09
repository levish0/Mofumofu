use crate::dto::report::response::report_info::ReportInfo;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GetReportsResponse {
    pub reports: Vec<ReportInfo>,
    pub total: u64,
    pub page: u64,
    pub per_page: u64,
}

impl IntoResponse for GetReportsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
