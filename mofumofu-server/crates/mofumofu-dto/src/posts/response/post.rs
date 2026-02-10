use chrono::{DateTime, Utc};
use mofumofu_entity::posts::Model as PostModel;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PostResponse {
    pub id: Uuid,
    pub user_id: Uuid,
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

impl PostResponse {
    pub fn from_model(model: PostModel, hashtags: Vec<String>) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            title: model.title,
            slug: model.slug,
            thumbnail_image: model.thumbnail_image,
            summary: model.summary,
            content: model.content,
            render: model.render,
            toc: model.toc,
            like_count: model.like_count,
            comment_count: model.comment_count,
            view_count: model.view_count,
            hashtags,
            published_at: model.published_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
