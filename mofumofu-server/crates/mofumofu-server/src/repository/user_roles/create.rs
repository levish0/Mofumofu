use mofumofu_entity::common::UserRole;
use mofumofu_entity::user_roles::{ActiveModel as UserRoleActiveModel, Model as UserRoleModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_user_role<C>(
    conn: &C,
    user_id: Uuid,
    role: UserRole,
    granted_by: Option<Uuid>,
) -> Result<UserRoleModel, Errors>
where
    C: ConnectionTrait,
{
    let new_user_role = UserRoleActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        role: Set(role),
        granted_by: Set(granted_by),
        created_at: Default::default(),
    };

    let user_role = new_user_role.insert(conn).await?;
    Ok(user_role)
}
