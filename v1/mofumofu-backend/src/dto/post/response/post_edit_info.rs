use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PostEditInfoResponse {
    pub id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
    pub thumbnail_image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub slug: String,
    pub tags: Vec<String>,
}

impl IntoResponse for PostEditInfoResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
