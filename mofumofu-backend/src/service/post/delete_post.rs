use crate::dto::post::request::delete_post::DeletePostRequest;
use crate::entity::common::{ActionType, TargetType};
use crate::microservices::search_client;
use crate::repository::post::delete_post::repository_delete_post;
use crate::repository::post::get_post_by_user_and_slug::repository_get_post_by_user_and_slug;
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use tracing::warn;
use uuid::Uuid;

pub async fn service_delete_post<C>(
    conn: &C,
    http_client: &reqwest::Client,
    payload: DeletePostRequest,
    user_uuid: &Uuid,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let post = repository_get_post_by_user_and_slug(conn, user_uuid, &payload.slug).await?;

    let txn = conn.begin().await?;

    repository_delete_post(&txn, &payload.slug, user_uuid).await?;

    txn.commit().await?;

    if let Err(e) = search_client::queue_delete_post(http_client, &post.id).await {
        warn!("Failed to queue post search deletion task: {}", e);
    }

    repository_log_event(
        conn,
        Some(*user_uuid),
        ActionType::PostDeleted,
        Some(post.id),
        Some(TargetType::Post),
        None,
    )
    .await;

    Ok(())
}
