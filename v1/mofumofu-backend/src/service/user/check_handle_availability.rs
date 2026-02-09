use crate::repository::user::find_user_by_handle::repository_find_user_by_handle;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;

pub async fn service_check_handle_availability<C>(conn: &C, handle: &str) -> ServiceResult<bool>
where
    C: ConnectionTrait,
{
    let user = repository_find_user_by_handle(conn, handle).await?;
    Ok(user.is_none())
}
