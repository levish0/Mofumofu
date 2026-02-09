use crate::dto::auth::request::verify_email::VerifyEmailRequest;
use crate::dto::user::internal::update_user::UpdateUserFields;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::repository::user::update_user::repository_update_user;
use crate::service::auth::jwt::decode_email_verification_token;
use crate::service::error::errors::{Errors, ServiceResult};
use chrono::Utc;
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use tracing::{error, info};

pub async fn service_verify_email<C>(conn: &C, payload: VerifyEmailRequest) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    // JWT 토큰 검증
    let token_data = decode_email_verification_token(&payload.token).map_err(|e| {
        error!("Invalid email verification token: {}", e);
        Errors::TokenInvalidVerification
    })?;

    let claims = token_data.claims;

    // 토큰 만료 확인
    let now = Utc::now().timestamp();
    if claims.exp < now {
        return Err(Errors::TokenExpiredVerification);
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

    // 이미 인증된 사용자인지 확인
    if user.is_verified {
        return Err(Errors::EmailAlreadyVerified);
    }

    // 사용자를 인증됨으로 업데이트
    let update_fields = UpdateUserFields {
        name: None,
        handle: None,
        bio: None,
        location: None,
        website: None,
        email: None,
        password: None,
        is_verified: Some(true),
        profile_image: None,
        banner_image: None,
    };

    repository_update_user(&txn, &claims.sub, update_fields).await?;

    txn.commit().await?;

    info!("Email verified successfully for user: {}", claims.sub);

    Ok(())
}
