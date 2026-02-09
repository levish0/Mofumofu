use crate::dto::auth::request::login::AuthLoginRequest;
use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::entity::common::{ActionType, TargetType};
use crate::entity::user_refresh_tokens::ActiveModel as RefreshTokenActiveModel;
use crate::repository::auth::create_refresh_token::repository_create_refresh_token;
use crate::repository::system_events::log_event::repository_log_event;
use crate::repository::user::find_user_by_handle::repository_find_user_by_handle;
use crate::service::auth::jwt::{create_jwt_access_token, create_jwt_refresh_token};
use crate::service::error::errors::{Errors, ServiceResult};
use crate::utils::crypto::verify_password;
use sea_orm::{ConnectionTrait, DatabaseConnection, Set, TransactionTrait};

pub async fn service_sign_in<C>(
    conn: &C,
    user_agent: Option<String>,
    ip_address: Option<String>,
    payload: AuthLoginRequest,
) -> ServiceResult<AuthJWTResponse>
where
    C: ConnectionTrait + TransactionTrait,
{
    let user = repository_find_user_by_handle(conn, &payload.handle)
        .await?
        .ok_or(Errors::UserNotFound)?;

    let stored_password = user.password.as_ref().ok_or(Errors::UserInvalidPassword)?;
    verify_password(&payload.password, stored_password)?;

    let access_token =
        create_jwt_access_token(&user.id).map_err(|e| Errors::TokenCreationError(e.to_string()))?;

    let refresh_token = create_jwt_refresh_token(&user.id)
        .map_err(|e| Errors::TokenCreationError(e.to_string()))?;

    let refresh_model = RefreshTokenActiveModel {
        id: Set(refresh_token.jti),
        user_id: Set(user.id),
        ip_address: Set(ip_address),
        user_agent: Set(user_agent),
        refresh_token: Set(refresh_token.token.clone()),
        expires_at: Set(refresh_token.expires_at),
        created_at: Set(refresh_token.issued_at),
        revoked_at: Default::default(),
    };

    let result = repository_create_refresh_token(conn, refresh_model)
        .await
        .map(|_| AuthJWTResponse {
            access_token,
            cookie_refresh_token: refresh_token.token,
        })
        .map_err(|e| e);

    // 로그인 성공 시 이벤트 로깅
    if result.is_ok() {
        repository_log_event(
            conn,
            Some(user.id),
            ActionType::UserSignedIn,
            Some(user.id),
            Some(TargetType::User),
            None,
        )
        .await;
    }

    result
}
