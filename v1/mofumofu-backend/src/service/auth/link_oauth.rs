use crate::dto::auth::request::link_oauth::LinkOAuthRequest;
use crate::entity::common::OAuthProvider;
use crate::repository::oauth::create_oauth_connection::repository_create_oauth_connection;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::service::oauth::provider::github::client::{exchange_github_code, get_github_user_info};
use crate::service::oauth::provider::google::client::{
    exchange_google_code_for_linking, get_google_user_info,
};
use crate::state::AppState;
use sea_orm::TransactionTrait;
use tracing::{error, info, warn};
use uuid::Uuid;

pub async fn service_link_oauth(
    state: &AppState,
    user_id: Uuid,
    payload: LinkOAuthRequest,
) -> ServiceResult<()> {
    let txn = state.conn.begin().await?;

    // 사용자 확인
    let _user = repository_find_user_by_uuid(&txn, &user_id)
        .await?
        .ok_or(Errors::UserNotFound)?;

    let provider = payload.provider.clone();

    // OAuth provider별로 사용자 정보 가져오기
    info!(
        "Attempting to link {:?} OAuth for user: {}",
        provider, user_id
    );
    info!("Authorization code length: {}", payload.code.len());

    let oauth_user_id = match payload.provider {
        OAuthProvider::Google => {
            info!("Starting Google OAuth token exchange for linking");
            let access_token = exchange_google_code_for_linking(&payload.code)
                .await
                .map_err(|e| {
                    error!("Google token exchange failed during linking: {:?}", e);
                    e
                })?;
            info!("Google token exchange successful, fetching user info");

            let user_info = get_google_user_info(&state.http_client, &access_token)
                .await
                .map_err(|e| {
                    error!("Google user info fetch failed during linking: {:?}", e);
                    e
                })?;
            info!("Google user info fetched successfully: {}", user_info.sub);
            user_info.sub
        }
        OAuthProvider::Github => {
            info!("Starting GitHub OAuth token exchange for linking");
            let access_token = exchange_github_code(&payload.code).await.map_err(|e| {
                error!("GitHub token exchange failed during linking: {:?}", e);
                e
            })?;
            info!("GitHub token exchange successful, fetching user info");

            let user_info = get_github_user_info(&state.http_client, &access_token)
                .await
                .map_err(|e| {
                    error!("GitHub user info fetch failed during linking: {:?}", e);
                    e
                })?;
            info!("GitHub user info fetched successfully: {}", user_info.id);
            user_info.id.to_string()
        }
    };

    // OAuth 연결 생성 (이미 연결되어 있으면 오류 발생)
    info!(
        "Creating OAuth connection for user: {} with {:?} ID: {}",
        user_id, provider, oauth_user_id
    );
    repository_create_oauth_connection(&txn, &user_id, payload.provider, &oauth_user_id)
        .await
        .map_err(|e| {
            error!("Failed to create OAuth connection: {:?}", e);
            Errors::OauthAccountAlreadyLinked
        })?;

    txn.commit().await?;

    info!(
        "OAuth {:?} account linked successfully for user: {}",
        provider, user_id
    );

    Ok(())
}
