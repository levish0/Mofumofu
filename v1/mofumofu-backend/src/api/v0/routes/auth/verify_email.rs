use crate::dto::auth::request::verify_email::VerifyEmailRequest;
use crate::service::auth::service_verify_email;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/auth/verify_email",
    request_body = VerifyEmailRequest,
    responses(
        (status = 200, description = "Email verified successfully"),
        (status = 400, description = "Token errors: token:invalid_verification, token:expired_verification, token:email_mismatch, email:already_verified"),
        (status = 404, description = "User not found"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth"
)]
pub async fn verify_email(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<VerifyEmailRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received POST request to verify email");

    service_verify_email(&state.conn, payload).await?;

    Ok(StatusCode::OK)
}
