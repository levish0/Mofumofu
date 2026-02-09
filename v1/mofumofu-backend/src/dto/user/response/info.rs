use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UserInfoResponse {
    pub name: String,
    pub handle: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<String>,

    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

impl IntoResponse for UserInfoResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
