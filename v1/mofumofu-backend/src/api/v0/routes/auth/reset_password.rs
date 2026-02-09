use crate::dto::auth::request::reset_password::ResetPasswordRequest;
use crate::service::auth::service_reset_password;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/auth/reset_password",
    request_body = ResetPasswordRequest,
    responses(
        (status = 200, description = "Password reset successfully"),
        (status = 400, description = "Token errors: token:invalid_reset, token:expired_reset, token:email_mismatch"),
        (status = 404, description = "User not found"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth"
)]
pub async fn reset_password(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<ResetPasswordRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received POST request to reset_password");

    service_reset_password(&state.conn, payload).await?;

    Ok(StatusCode::OK)
}
