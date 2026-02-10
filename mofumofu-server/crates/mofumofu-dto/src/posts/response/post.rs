use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostAuthor {
    pub id: Uuid,
    pub handle: String,
    pub display_name: String,
    pub profile_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub author: PostAuthor,
    pub title: String,
    pub slug: String,
    pub thumbnail_image: Option<String>,
    pub summary: Option<String>,
    pub content: String,
    pub render: Option<String>,
    #[schema(value_type = Option<Object>)]
    pub toc: Option<JsonValue>,
    pub like_count: i32,
    pub comment_count: i32,
    pub view_count: i32,
    pub hashtags: Vec<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
