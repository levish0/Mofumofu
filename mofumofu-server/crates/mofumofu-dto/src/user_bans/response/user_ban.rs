use chrono::{DateTime, Utc};
use mofumofu_entity::user_bans::Model as UserBanModel;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UserBanResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub banned_by: Uuid,
    pub reason: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl From<UserBanModel> for UserBanResponse {
    fn from(model: UserBanModel) -> Self {
        Self {
            id: model.id,
            user_id: model.user_id,
            banned_by: model.banned_by,
            reason: model.reason,
            expires_at: model.expires_at,
            created_at: model.created_at,
        }
    }
}
