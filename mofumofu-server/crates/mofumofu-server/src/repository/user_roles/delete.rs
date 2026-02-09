use mofumofu_entity::common::UserRole;
use mofumofu_entity::user_roles::{Column as UserRoleColumn, Entity as UserRoleEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, DeleteResult, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_delete_user_role<C>(
    conn: &C,
    user_id: Uuid,
    role: UserRole,
) -> Result<DeleteResult, Errors>
where
    C: ConnectionTrait,
{
    let result = UserRoleEntity::delete_many()
        .filter(UserRoleColumn::UserId.eq(user_id))
        .filter(UserRoleColumn::Role.eq(role))
        .exec(conn)
        .await?;

    Ok(result)
}
