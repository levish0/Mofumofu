use chrono::{DateTime, Utc};
use mofumofu_entity::comments::Model as CommentModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CommentResponse {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub depth: i32,
    pub content: String,
    pub like_count: i32,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<CommentModel> for CommentResponse {
    fn from(model: CommentModel) -> Self {
        Self {
            id: model.id,
            post_id: model.post_id,
            user_id: model.user_id,
            parent_id: model.parent_id,
            depth: model.depth,
            content: model.content,
            like_count: model.like_count,
            deleted_at: model.deleted_at,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
