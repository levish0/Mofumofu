use crate::dto::auth::request::reset_password::ResetPasswordRequest;
use crate::dto::user::internal::update_user::UpdateUserFields;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::repository::user::update_user::repository_update_user;
use crate::service::auth::jwt::decode_password_reset_token;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::utils::crypto::hash_password;
use chrono::Utc;
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use tracing::{error, info};

pub async fn service_reset_password<C>(conn: &C, payload: ResetPasswordRequest) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    // JWT 토큰 검증
    let token_data = decode_password_reset_token(&payload.token).map_err(|e| {
        error!("Invalid password reset token: {}", e);
        Errors::TokenInvalidReset
    })?;

    let claims = token_data.claims;

    // 토큰 만료 확인
    let now = Utc::now().timestamp();
    if claims.exp < now {
        return Err(Errors::TokenExpiredReset);
    }

    let txn = conn.begin().await?;

    // 사용자 확인
    let user = repository_find_user_by_uuid(&txn, &claims.sub)
        .await?
        .ok_or(Errors::UserNotFound)?;

    // 이메일이 일치하는지 확인
    if user.email != claims.email {
        return Err(Errors::TokenEmailMismatch);
    }

    // 새 비밀번호 해싱
    let hashed_password = hash_password(&payload.new_password)?;

    // 사용자 비밀번호 업데이트
    let update_fields = UpdateUserFields {
        name: None,
        handle: None,
        bio: None,
        location: None,
        website: None,
        email: None,
        password: Some(Some(hashed_password)),
        is_verified: None,
        profile_image: None,
        banner_image: None,
    };

    repository_update_user(&txn, &claims.sub, update_fields).await?;

    txn.commit().await?;

    info!("Password reset successfully for user: {}", claims.sub);

    Ok(())
}
