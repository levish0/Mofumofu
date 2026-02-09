use crate::dto::like::response::like_status::LikeStatusResponse;
use crate::repository::like::check_like_status::repository_check_like_status_by_comment_id;
use crate::service::error::errors::ServiceResult;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_check_comment_like_status<C>(
    conn: &C,
    user_id: &Uuid,
    comment_id: &Uuid,
) -> ServiceResult<LikeStatusResponse>
where
    C: ConnectionTrait,
{
    let is_liked = repository_check_like_status_by_comment_id(conn, user_id, comment_id).await?;

    Ok(LikeStatusResponse { is_liked })
}
