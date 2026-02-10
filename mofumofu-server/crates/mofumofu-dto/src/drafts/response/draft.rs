use chrono::{DateTime, Utc};
use mofumofu_entity::drafts::Model as DraftModel;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DraftResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    #[schema(value_type = Option<Object>)]
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<DraftModel> for DraftResponse {
    fn from(model: DraftModel) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            title: model.title,
            slug: model.slug,
            content: model.content,
            metadata: model.metadata,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
