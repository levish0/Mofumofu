use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

use super::HashtagResponse;

#[derive(Debug, Serialize, ToSchema)]
pub struct TrendingHashtagsResponse {
    pub data: Vec<HashtagResponse>,
}

impl IntoResponse for TrendingHashtagsResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
