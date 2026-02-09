use crate::entity::users::{Column, Entity as UserEntity, Model as UserModel};
use crate::repository::user::find_user_by_handle::repository_find_user_by_handle;
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

pub async fn repository_get_user_by_handle<C>(conn: &C, handle: &str) -> Result<UserModel, Errors>
where
    C: ConnectionTrait,
{
    repository_find_user_by_handle(conn, handle)
        .await?
        .ok_or(Errors::UserNotFound)
}
