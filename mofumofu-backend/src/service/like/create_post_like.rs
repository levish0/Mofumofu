use crate::entity::common::{ActionType, TargetType};
use crate::repository::like::check_like_status::repository_check_like_status_by_post_id;
use crate::repository::like::create_like::repository_create_like_by_post_id;
use crate::repository::post::get_post_by_uuid::repository_get_post_by_uuid;
use crate::repository::post::update_like_count::repository_increment_post_like_count;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use uuid::Uuid;

pub async fn service_create_post_like<C>(
    conn: &C,
    user_id: &Uuid,
    post_id: &Uuid,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // 포스트 존재 확인
    let post = repository_get_post_by_uuid(&txn, post_id).await?;

    // 자신의 포스트에도 좋아요를 누를 수 있음 (제거된 제약)

    // 이미 좋아요가 있는지 확인
    let already_liked = repository_check_like_status_by_post_id(&txn, user_id, post_id).await?;
    if already_liked {
        return Err(Errors::LikeAlreadyExists);
    }

    // 좋아요 생성
    let _created_like = repository_create_like_by_post_id(&txn, *user_id, *post_id).await?;

    // 포스트 좋아요 개수 증가
    repository_increment_post_like_count(&txn, *post_id).await?;

    txn.commit().await?;

    // 좋아요 생성 이벤트 로깅
    repository_log_event(
        conn,
        Some(*user_id),
        ActionType::LikeCreated,
        Some(*post_id),
        Some(TargetType::Post),
        None,
    )
    .await;

    Ok(())
}
