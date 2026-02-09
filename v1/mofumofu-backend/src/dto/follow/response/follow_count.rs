use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct FollowCountResponse {
    pub count: u64,
}

impl IntoResponse for FollowCountResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
