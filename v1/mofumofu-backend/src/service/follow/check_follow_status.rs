use crate::repository::follow::check_follow_exists::repository_check_follow_exists;
use crate::repository::user::find_user_by_handle::repository_find_user_by_handle;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_check_follow_status<C>(
    conn: &C,
    follower_id: &Uuid,
    followee_handle: &str,
) -> ServiceResult<bool>
where
    C: ConnectionTrait,
{
    let follower = repository_find_user_by_uuid(conn, follower_id)
        .await?
        .ok_or(Errors::UserNotFound)?;

    let followee = repository_find_user_by_handle(conn, followee_handle)
        .await?
        .ok_or(Errors::UserNotFound)?;

    let is_following = repository_check_follow_exists(conn, follower.id, followee.id).await?;

    Ok(is_following)
}
