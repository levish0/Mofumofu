use crate::dto::auth::request::forgot_password::ForgotPasswordRequest;
use crate::service::auth::service_forgot_password;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/auth/forgot_password",
    request_body = ForgotPasswordRequest,
    responses(
        (status = 200, description = "Password reset email sent if account exists"),
        (status = 400, description = "Invalid email format"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth"
)]
pub async fn forgot_password(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<ForgotPasswordRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received POST request to forgot_password: {:?}",
        payload.email
    );

    service_forgot_password(&state, payload).await?;

    Ok(StatusCode::OK)
}
