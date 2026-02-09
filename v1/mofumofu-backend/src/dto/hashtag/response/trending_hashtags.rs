use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct TrendingHashtagsResponse {
    pub hashtags: Vec<String>,
}

impl IntoResponse for TrendingHashtagsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
