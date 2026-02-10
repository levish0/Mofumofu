use chrono::{DateTime, Utc};
use mofumofu_entity::common::ModerationResourceType;
use mofumofu_entity::moderation_logs::Model as ModerationLogModel;
use serde::Serialize;
use serde_json::Value as JsonValue;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ModerationLogResponse {
    pub id: Uuid,
    pub action: String,
    pub actor_id: Uuid,
    pub resource_type: ModerationResourceType,
    pub resource_id: Option<Uuid>,
    pub reason: Option<String>,
    #[schema(value_type = Option<Object>)]
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
}

impl From<ModerationLogModel> for ModerationLogResponse {
    fn from(model: ModerationLogModel) -> Self {
        Self {
            id: model.id,
            action: model.action,
            actor_id: model.actor_id,
            resource_type: model.resource_type,
            resource_id: model.resource_id,
            reason: model.reason,
            metadata: model.metadata,
            created_at: model.created_at,
        }
    }
}
