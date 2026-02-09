use crate::entity::common::{ActionType, TargetType};
use crate::repository::post::get_post_by_uuid::repository_get_post_by_uuid;
use crate::repository::post::increment_view_count::repository_increment_view_count;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::state::AppState;
use redis::AsyncCommands;
use sea_orm::ConnectionTrait;
use tracing::{error, info};
use uuid::Uuid;

const VIEW_COUNT_TTL: i64 = 3600; // 1시간

pub async fn service_increment_view<C>(
    app_state: &AppState,
    conn: &C,
    post_id: &Uuid,
    anonymous_user_id: Option<&str>,
) -> ServiceResult<()>
where
    C: ConnectionTrait,
{
    // 포스트 존재 여부 확인
    let _post = repository_get_post_by_uuid(conn, post_id).await?;

    // 조회수 증가
    increment_view_count_with_redis_check(app_state, conn, post_id, anonymous_user_id).await?;

    Ok(())
}

pub async fn increment_view_count_with_redis_check<C>(
    app_state: &AppState,
    conn: &C,
    post_id: &Uuid,
    anonymous_user_id: Option<&str>,
) -> ServiceResult<()>
where
    C: ConnectionTrait,
{
    let mut redis_conn = app_state.redis.clone();

    // Redis 키 생성 (anonymous_user_id만 사용)
    let redis_key = match anonymous_user_id {
        Some(anon_id) => format!("view:{}:anon:{}", post_id, anon_id),
        None => {
            error!("anonymous_user_id is None");
            return Err(Errors::SysInternalError("".to_string()));
        }
    };

    // Redis에서 키 존재 여부 확인
    let exists: bool = redis_conn.exists(&redis_key).await.map_err(|e| {
        error!("Redis exists check failed: {}", e);
        Errors::SysInternalError("".to_string())
    })?;

    if !exists {
        // DB에서 조회수 증가
        repository_increment_view_count(conn, post_id).await?;

        repository_log_event(
            conn,
            None, // 익명 사용자이므로 user_id는 None
            ActionType::PostViewed,
            Some(*post_id),
            Some(TargetType::Post),
            None,
        )
        .await;

        // 중복 방지를 위한 키 저장 (TTL 설정)
        let _: () = redis_conn
            .set_ex(&redis_key, "1", VIEW_COUNT_TTL as u64)
            .await
            .map_err(|e| {
                error!("Failed to set Redis key with TTL: {}", e);
                Errors::SysInternalError("".to_string())
            })?;

        info!(
            "View count incremented for post: {} and logged system event",
            post_id
        );
    } else {
        info!("View already counted for post: {} within TTL", post_id);
    }

    Ok(())
}
