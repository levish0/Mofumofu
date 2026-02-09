use crate::dto::user::response::info::UserInfoResponse;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct FollowListResponse {
    pub users: Vec<UserInfoResponse>,
    pub total_count: u64,
    pub page: u64,
    pub per_page: u64,
    pub has_more: bool,
}

impl IntoResponse for FollowListResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
