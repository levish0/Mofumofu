use axum::Json;
use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct PostListItem {
    pub id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub thumbnail_image: Option<String>,
    pub user_handle: String,
    pub user_name: String,
    pub user_avatar: Option<String>,
    pub created_at: DateTime<Utc>,
    pub like_count: i32,
    pub comment_count: i32,
    pub view_count: i32,
    pub slug: String,
    pub hashtags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct GetPostsResponse {
    pub posts: Vec<PostListItem>,
    pub current_page: u32,
    pub page_size: u32,
    pub has_more: bool,
    pub total_count: u64,
}

impl IntoResponse for GetPostsResponse {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
