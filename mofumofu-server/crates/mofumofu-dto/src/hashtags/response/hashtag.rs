use chrono::{DateTime, Utc};
use mofumofu_entity::hashtags::Model as HashtagModel;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct HashtagResponse {
    pub id: Uuid,
    pub name: String,
    pub usage_count: i32,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<HashtagModel> for HashtagResponse {
    fn from(model: HashtagModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            usage_count: model.usage_count,
            last_used_at: model.last_used_at,
            created_at: model.created_at,
        }
    }
}
