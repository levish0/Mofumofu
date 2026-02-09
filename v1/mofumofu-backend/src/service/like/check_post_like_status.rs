use crate::dto::like::response::like_status::LikeStatusResponse;
use crate::repository::like::check_like_status::repository_check_like_status_by_post_id;
use crate::service::error::errors::ServiceResult;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_check_post_like_status<C>(
    conn: &C,
    user_id: &Uuid,
    post_id: &Uuid,
) -> ServiceResult<LikeStatusResponse>
where
    C: ConnectionTrait,
{
    let is_liked = repository_check_like_status_by_post_id(conn, user_id, post_id).await?;

    Ok(LikeStatusResponse { is_liked })
}
