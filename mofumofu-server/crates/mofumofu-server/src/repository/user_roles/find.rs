use mofumofu_entity::user_roles::{
    Column as UserRoleColumn, Entity as UserRoleEntity, Model as UserRoleModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_find_user_roles<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<Vec<UserRoleModel>, Errors>
where
    C: ConnectionTrait,
{
    let roles = UserRoleEntity::find()
        .filter(UserRoleColumn::UserId.eq(user_id))
        .all(conn)
        .await?;

    Ok(roles)
}
