use crate::dto::auth::request::resend_verification::ResendVerificationRequest;
use crate::service::auth::service_resend_verification;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/auth/resend_verification",
    request_body = ResendVerificationRequest,
    responses(
        (status = 200, description = "Verification email sent successfully"),
        (status = 400, description = "Email already verified: email:already_verified"),
        (status = 404, description = "User not found"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth"
)]
pub async fn resend_verification(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<ResendVerificationRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received POST request to resend verification email for: {}",
        payload.email
    );

    service_resend_verification(&state, &state.conn, payload).await?;

    Ok(StatusCode::OK)
}
