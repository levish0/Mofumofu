use crate::entity::users::{Entity as UserEntity, Model as UserModel};
use crate::service::error::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait};
use uuid::Uuid;

pub async fn repository_find_user_by_uuid<C>(
    conn: &C,
    user_uuid: &Uuid,
) -> Result<Option<UserModel>, Errors>
where
    C: ConnectionTrait,
{
    Ok(UserEntity::find_by_id(*user_uuid).one(conn).await?)
}
