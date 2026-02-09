use crate::entity::users::{Column, Entity as UserEntity, Model as UserModel};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

pub async fn repository_find_user_by_handle<C>(
    conn: &C,
    handle: &str,
) -> Result<Option<UserModel>, Errors>
where
    C: ConnectionTrait,
{
    Ok(UserEntity::find()
        .filter(Column::Handle.eq(handle))
        .one(conn)
        .await?)
}
