use crate::dto::auth::request::resend_verification::ResendVerificationRequest;
use crate::microservices::email_client::queue_send_email_verification;
use crate::repository::user::find_user_by_email::repository_find_user_by_email;
use crate::service::auth::jwt::create_email_verification_token;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::state::AppState;
use sea_orm::ConnectionTrait;
use tracing::{error, info};

pub async fn service_resend_verification<C>(
    state: &AppState,
    conn: &C,
    payload: ResendVerificationRequest,
) -> ServiceResult<()>
where
    C: ConnectionTrait,
{
    // 사용자 찾기
    let user = repository_find_user_by_email(conn, &payload.email)
        .await?
        .ok_or(Errors::UserNotFound)?;

    // 이미 인증된 사용자인지 확인
    if user.is_verified {
        return Err(Errors::EmailAlreadyVerified);
    }

    // 이메일 인증 토큰 생성
    let verification_token =
        create_email_verification_token(&user.id, &user.email).map_err(|e| {
            error!("Failed to create email verification token: {}", e);
            Errors::SysInternalError("Failed to create verification token".to_string())
        })?;

    // 비동기로 이메일 발송 요청
    if let Err(e) = queue_send_email_verification(
        &state.http_client,
        &user.email,
        &user.name,
        &verification_token,
    )
    .await
    {
        error!("Failed to queue verification email: {}", e);
        return Err(Errors::SysInternalError(
            "Failed to send verification email".to_string(),
        ));
    } else {
        info!("Verification email queued for user: {}", user.email);
    }

    Ok(())
}
