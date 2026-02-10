use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;

use super::FollowUserItem;

#[derive(Debug, Serialize, ToSchema)]
pub struct FollowListResponse {
    pub data: Vec<FollowUserItem>,
    pub has_newer: bool,
    pub has_older: bool,
}

impl IntoResponse for FollowListResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
