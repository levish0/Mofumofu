use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ThumbnailUploadResponse {
    pub public_url: String,
}

impl IntoResponse for ThumbnailUploadResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
