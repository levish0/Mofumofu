use axum::Json;
use axum::response::IntoResponse;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PostFeedItem {
    pub id: Uuid,
    pub user_id: Uuid,
    pub author_handle: String,
    pub author_display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_profile_image: Option<String>,
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_image: Option<String>,
    pub hashtags: Vec<String>,
    pub like_count: i32,
    pub comment_count: i32,
    pub view_count: i32,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PostFeedResponse {
    pub data: Vec<PostFeedItem>,
    pub page: u32,
    pub page_size: u32,
    pub has_more: bool,
    pub total_count: u64,
}

impl IntoResponse for PostFeedResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
