use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DraftInfo {
    pub draft_id: Uuid,
    pub title: Option<String>,
    pub thumbnail_image: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl IntoResponse for DraftInfo {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(self)).into_response()
    }
}
