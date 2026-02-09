use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct CommentInfo {
    pub id: Uuid,
    pub content: Option<String>,
    pub post_id: Uuid,
    pub user_id: Option<Uuid>,
    pub user_handle: Option<String>,
    pub user_name: Option<String>,
    pub user_profile_image: Option<String>,
    pub parent_id: Option<Uuid>,
    pub like_count: i32,
    pub reply_count: i32,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl IntoResponse for CommentInfo {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
