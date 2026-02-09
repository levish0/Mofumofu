use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AdminTaskResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl IntoResponse for AdminTaskResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
