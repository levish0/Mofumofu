use crate::dto::auth::request::forgot_password::ForgotPasswordRequest;
use crate::microservices::email_client::queue_send_reset_password_email;
use crate::repository::user::find_user_by_email::repository_find_user_by_email;
use crate::service::auth::jwt::create_password_reset_token;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::state::AppState;
use tracing::{error, info};

pub async fn service_forgot_password(
    state: &AppState,
    payload: ForgotPasswordRequest,
) -> ServiceResult<()> {
    // 사용자 조회 (보안상 이메일 존재 여부는 노출하지 않음)
    let user_result = repository_find_user_by_email(&state.conn, &payload.email).await;

    match user_result {
        Ok(Some(user)) => {
            // 비밀번호 재설정 토큰 생성
            let reset_token = create_password_reset_token(&user.id, &user.email).map_err(|e| {
                error!("Failed to create password reset token: {}", e);
                Errors::SysInternalError("Failed to create reset token".to_string())
            })?;

            // 비동기로 비밀번호 재설정 이메일 발송 요청
            if let Err(e) = queue_send_reset_password_email(
                &state.http_client,
                &user.email,
                &user.email,
                &reset_token,
            )
            .await
            {
                error!("Failed to queue password reset email: {}", e);
                // 이메일 발송 실패는 로그만 남기고 성공으로 처리 (보안상)
            } else {
                info!("Password reset email queued for user: {}", user.email);
            }
        }
        Ok(None) => {
            // 사용자가 존재하지 않음 - 보안상 동일한 응답
            info!(
                "Password reset requested for non-existent email: {}",
                payload.email
            );
        }
        Err(e) => {
            error!("Database error during forgot password: {:?}", e);
            return Err(e);
        }
    }

    // 보안상 이메일 존재 여부와 관계없이 항상 성공 응답
    Ok(())
}
