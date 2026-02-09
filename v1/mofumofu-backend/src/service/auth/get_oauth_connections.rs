use crate::dto::auth::response::oauth_connections::OAuthConnectionsResponse;
use crate::repository::oauth::get_user_oauth_connections::repository_get_oauth_providers_by_user_id;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_get_oauth_connections<C>(
    conn: &C,
    user_id: &Uuid,
) -> ServiceResult<OAuthConnectionsResponse>
where
    C: ConnectionTrait,
{
    // 사용자 정보 조회
    let user = repository_find_user_by_uuid(conn, user_id)
        .await?
        .ok_or(Errors::UserNotFound)?;

    // OAuth 연결 목록 조회
    let connections = repository_get_oauth_providers_by_user_id(conn, user_id).await?;

    // OAuth 전용 계정인지 확인 (비밀번호가 없고 OAuth 연결이 있는 경우)
    let is_oauth_only = user.password.is_none() && !connections.is_empty();

    Ok(OAuthConnectionsResponse {
        connections,
        is_oauth_only,
    })
}
