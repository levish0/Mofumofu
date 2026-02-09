use crate::repository::follow::get_following_count::repository_get_following_count;
use crate::repository::user::get_user_by_handle::repository_get_user_by_handle;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use sea_orm::TransactionTrait;

pub async fn service_get_following_count<C>(conn: &C, user_handle: &str) -> ServiceResult<u64>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = repository_get_user_by_handle(conn, user_handle).await?;
    let count = repository_get_following_count(conn, user.id).await?;
    Ok(count)
}
