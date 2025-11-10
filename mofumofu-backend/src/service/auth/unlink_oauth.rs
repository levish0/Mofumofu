use crate::dto::auth::request::unlink_oauth::UnlinkOAuthRequest;
use crate::repository::oauth::delete_oauth_connection::repository_delete_oauth_connection;
use crate::repository::oauth::get_user_oauth_connections::repository_get_oauth_providers_by_user_id;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use tracing::info;
use uuid::Uuid;

pub async fn service_unlink_oauth<C>(
    conn: &C,
    user_id: &Uuid,
    payload: UnlinkOAuthRequest,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // 사용자 확인
    let user = repository_find_user_by_uuid(&txn, user_id)
        .await?
        .ok_or(Errors::UserNotFound)?;

    // 현재 OAuth 연결 목록 조회
    let current_connections = repository_get_oauth_providers_by_user_id(&txn, user_id).await?;

    // OAuth 전용 계정인지 확인
    let is_oauth_only = user.password.is_none();

    // OAuth 전용 계정에서 마지막 연결을 해제하려는 경우 방지
    if is_oauth_only && current_connections.len() <= 1 {
        return Err(Errors::OauthCannotUnlinkLastConnection);
    }

    // 해당 provider가 실제로 연결되어 있는지 확인
    if !current_connections.contains(&payload.provider) {
        return Err(Errors::OauthConnectionNotFound);
    }

    // OAuth 연결 해제
    repository_delete_oauth_connection(&txn, user_id, &payload.provider).await?;

    txn.commit().await?;

    info!(
        "OAuth {:?} account unlinked successfully for user: {}",
        payload.provider, user_id
    );

    Ok(())
}
