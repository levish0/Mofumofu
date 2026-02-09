use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateReportResponse {
    pub report_id: Uuid,
}

impl IntoResponse for CreateReportResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
