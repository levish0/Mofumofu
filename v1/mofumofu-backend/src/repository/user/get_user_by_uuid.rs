use crate::entity::users::{Entity as UserEntity, Model as UserModel};
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::Errors;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn repository_get_user_by_uuid<C>(conn: &C, user_uuid: &Uuid) -> Result<UserModel, Errors>
where
    C: ConnectionTrait,
{
    repository_find_user_by_uuid(conn, user_uuid)
        .await?
        .ok_or(Errors::UserNotFound)
}
