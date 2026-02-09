use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::auth::request::set_password::SetPasswordRequest;
use crate::service::auth::service_set_password;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/auth/set_password",
    request_body = SetPasswordRequest,
    responses(
        (status = 200, description = "Password set successfully"),
        (status = 400, description = "Password already set: password:already_set"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth",
    security(("bearer_auth" = []))
)]
pub async fn set_password(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<SetPasswordRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received POST request to set password for user: {}",
        claims.sub
    );

    service_set_password(&state.conn, claims.sub, payload).await?;

    Ok(StatusCode::OK)
}
