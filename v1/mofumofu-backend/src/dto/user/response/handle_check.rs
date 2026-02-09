use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct HandleCheckResponse {
    pub is_available: bool,
}

impl IntoResponse for HandleCheckResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
