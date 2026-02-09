use crate::dto::oauth::internal::oauth_user_result::OAuthUserResult;
use crate::entity::common::OAuthProvider;
use crate::entity::users::Model as UserModel;
use crate::repository::oauth::create_oauth_connection::repository_create_oauth_connection;
use crate::repository::oauth::create_oauth_user::repository_create_oauth_user;
use crate::repository::oauth::find_user_by_oauth::repository_find_user_by_oauth;
use crate::repository::user::find_user_by_email::repository_find_user_by_email;
use crate::repository::user::find_user_by_handle::repository_find_user_by_handle;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::{error, info};

pub async fn service_find_or_create_oauth_user<C>(
    txn: &C,
    email: &str,
    name: &str,
    provider_id: &str,
    provider: OAuthProvider,
    profile_image: Option<String>,
    handle: Option<&str>,
) -> ServiceResult<OAuthUserResult>
where
    C: ConnectionTrait + TransactionTrait,
{
    if let Some(existing_user) =
        repository_find_user_by_oauth(txn, provider.clone(), provider_id).await?
    {
        info!(
            "Found existing user via OAuth: {} for provider: {:?}",
            existing_user.email, provider
        );
        return Ok(OAuthUserResult {
            user: existing_user,
            is_new_user: false,
        });
    }

    // 2. 이메일로 기존 유저 찾기
    if let Some(existing_user) = repository_find_user_by_email(txn, email).await? {
        info!(
            "Found existing user with email: {}, creating OAuth connection for provider: {:?}",
            email, provider
        );

        // 기존 유저에게 OAuth 연결 추가
        repository_create_oauth_connection(txn, &existing_user.id, provider, provider_id).await?;

        return Ok(OAuthUserResult {
            user: existing_user,
            is_new_user: false,
        });
    }

    // 3. handle이 없으면 로그인 시도인데 유저가 없으므로 에러
    let handle = handle.ok_or(Errors::UserNotFound)?;

    // 4. 핸들 중복 확인 (가입 시에만)
    if repository_find_user_by_handle(txn, handle).await?.is_some() {
        return Err(Errors::UserHandleAlreadyExists);
    }

    // 5. 새 유저 생성
    repository_create_oauth_user(txn, email, name, handle, profile_image).await?;
    // 6. 생성된 유저 조회
    let created_user = repository_find_user_by_email(txn, email)
        .await?
        .ok_or_else(|| {
            error!("Failed to retrieve newly created OAuth user");
            Errors::DatabaseError("User creation verification failed".to_string())
        })?;
    // 7. OAuth 연결 생성
    repository_create_oauth_connection(txn, &created_user.id, provider.clone(), provider_id)
        .await?;

    info!(
        "Created new OAuth user: {} ({}) via {:?} with provider_id: {}",
        created_user.email, created_user.handle, provider, provider_id
    );

    Ok(OAuthUserResult {
        user: created_user,
        is_new_user: true,
    })
}
