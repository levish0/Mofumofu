use mofumofu_entity::common::UserRole;
use mofumofu_entity::user_roles::{Column as UserRoleColumn, Entity as UserRoleEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_get_highest_user_role<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<Option<UserRole>, Errors>
where
    C: ConnectionTrait,
{
    let roles = UserRoleEntity::find()
        .filter(UserRoleColumn::UserId.eq(user_id))
        .all(conn)
        .await?;

    let highest = roles
        .into_iter()
        .map(|r| r.role)
        .max_by_key(|r| r.priority());

    Ok(highest)
}
