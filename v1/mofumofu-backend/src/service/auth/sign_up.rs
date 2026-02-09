use crate::dto::user::request::create::CreateUserRequest;
use crate::microservices::email_client::queue_send_email_verification;
use crate::repository::user::create_user::repository_create_user;
use crate::service::auth::jwt::create_email_verification_token;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::state::AppState;
use sea_orm::TransactionTrait;
use tracing::{error, info};

pub async fn service_sign_up(state: &AppState, payload: CreateUserRequest) -> ServiceResult<()> {
    let txn = state.conn.begin().await?;

    // 사용자 생성
    let user = repository_create_user(&txn, payload).await?;

    txn.commit().await?;

    // 이메일 인증 토큰 생성
    let verification_token =
        create_email_verification_token(&user.id, &user.email).map_err(|e| {
            error!("Failed to create email verification token: {}", e);
            Errors::SysInternalError("Failed to create verification token".to_string())
        })?;

    // 비동기로 이메일 발송 요청 (실패해도 회원가입은 성공)
    if let Err(e) = queue_send_email_verification(
        &state.http_client,
        &user.email,
        &user.name,
        &verification_token,
    )
    .await
    {
        error!("Failed to queue verification email: {}", e);
        // 이메일 발송 실패는 로그만 남기고 회원가입은 성공으로 처리
    } else {
        info!("Verification email queued for user: {}", user.email);
    }

    Ok(())
}
