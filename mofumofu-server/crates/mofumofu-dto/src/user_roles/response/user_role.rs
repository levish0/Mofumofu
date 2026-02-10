use chrono::{DateTime, Utc};
use mofumofu_entity::common::UserRole;
use mofumofu_entity::user_roles::Model as UserRoleModel;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct UserRoleResponse {
    pub user_id: Uuid,
    pub role: UserRole,
    pub granted_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl From<UserRoleModel> for UserRoleResponse {
    fn from(model: UserRoleModel) -> Self {
        Self {
            user_id: model.user_id,
            role: model.role,
            granted_by: model.granted_by,
            created_at: model.created_at,
        }
    }
}
