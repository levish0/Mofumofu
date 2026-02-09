use crate::dto::auth::internal::refresh_token::RefreshTokenClaims;
use crate::dto::auth::response::jwt::AuthJWTResponse;
use crate::entity::user_refresh_tokens::ActiveModel as RefreshTokenActiveModel;
use crate::repository::auth::create_refresh_token::repository_create_refresh_token;
use crate::repository::auth::find_refresh_token_by_jti_and_token::repository_find_refresh_token_by_jti_and_token;
use crate::repository::auth::revoke_refresh_token::repository_revoke_refresh_token;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::auth::jwt::{create_jwt_access_token, create_jwt_refresh_token};
use crate::service::error::errors::{Errors, ServiceResult};
use chrono::Utc;
use sea_orm::{ConnectionTrait, DatabaseConnection, Set, TransactionTrait};

pub async fn service_refresh(
    conn: &DatabaseConnection,
    user_agent: Option<String>,
    ip_address: Option<String>,
    refresh_token: String,
    refresh_token_claims: RefreshTokenClaims,
) -> ServiceResult<AuthJWTResponse> {
    let now = Utc::now().timestamp();
    if refresh_token_claims.exp < now {
        return Err(Errors::UserTokenExpired);
    }

    let stored_token = repository_find_refresh_token_by_jti_and_token(
        conn,
        refresh_token_claims.jti,
        refresh_token,
    )
    .await?
    .ok_or(Errors::UserInvalidToken)?;

    let user = repository_find_user_by_uuid(conn, &refresh_token_claims.sub)
        .await?
        .ok_or(Errors::UserNotFound)?;

    repository_revoke_refresh_token(conn, stored_token, None, None, Utc::now()).await?;

    let new_access_token =
        create_jwt_access_token(&user.id).map_err(|e| Errors::TokenCreationError(e.to_string()))?;

    let new_refresh_token = create_jwt_refresh_token(&user.id)
        .map_err(|e| Errors::TokenCreationError(e.to_string()))?;

    let new_refresh_model = RefreshTokenActiveModel {
        id: Set(new_refresh_token.jti),
        user_id: Set(user.id),
        ip_address: Set(ip_address),
        user_agent: Set(user_agent),
        refresh_token: Set(new_refresh_token.token.clone()),
        expires_at: Set(new_refresh_token.expires_at),
        created_at: Set(new_refresh_token.issued_at),
        revoked_at: Default::default(),
    };

    repository_create_refresh_token(conn, new_refresh_model)
        .await
        .map(|_| AuthJWTResponse {
            access_token: new_access_token,
            cookie_refresh_token: new_refresh_token.token,
        })
        .map_err(|e| e)
}
