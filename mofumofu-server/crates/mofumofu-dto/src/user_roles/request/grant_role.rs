use mofumofu_entity::common::UserRole;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GrantRoleRequest {
    pub user_id: Uuid,
    pub role: UserRole,
}
