use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UploadPostImageResponse {
    pub image_url: String,
}

impl IntoResponse for UploadPostImageResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
