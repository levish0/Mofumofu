use crate::entity::users::Entity as UsersEntity;
use crate::entity::users::{GetFollowingLink, Model as UsersModel};
use crate::repository::user::get_user_by_handle::repository_get_user_by_handle;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use sea_orm::EntityTrait;
use sea_orm::QuerySelect;
use sea_orm::TransactionTrait;

pub async fn service_get_following<C>(
    conn: &C,
    user_handle: &str,
    offset: u64,
    limit: u64,
) -> ServiceResult<Vec<UsersModel>>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = repository_get_user_by_handle(conn, user_handle).await?;

    // Join을 사용한 더 효율적인 쿼리
    let following_with_user = UsersEntity::find_by_id(user.id)
        .find_also_linked(GetFollowingLink)
        .offset(offset)
        .limit(limit)
        .all(conn)
        .await?;

    // Extract the second element (following users) from each tuple and filter out None values
    let following: Vec<UsersModel> = following_with_user
        .into_iter()
        .filter_map(|(_, following_user)| following_user)
        .collect();

    Ok(following)
}
