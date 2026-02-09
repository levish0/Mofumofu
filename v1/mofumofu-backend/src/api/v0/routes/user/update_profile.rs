use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::user::request::update_profile::UpdateProfileRequest;
use crate::dto::user::response::info::UserInfoResponse;
use crate::service::auth::require_verified_user;
use crate::service::error::errors::Errors;
use crate::service::user::service_update_user_profile;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::{error, info, warn};

#[utoipa::path(
    put,
    path = "/v0/user/profile",
    request_body = UpdateProfileRequest,
    responses(
        (status = 200, description = "Profile updated successfully", body = UserInfoResponse),
        (status = 400, description = "Password related errors: password:required_for_update, password:incorrect, password:cannot_update_oauth_only, password:new_password_missing"),
        (status = 401, description = "Unauthorized or email not verified"),
        (status = 404, description = "User not found"),
        (status = 409, description = "Handle already exists"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "User"
)]
pub async fn update_profile(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<UpdateProfileRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received PUT request to update profile for user: {}",
        claims.sub
    );

    require_verified_user(&state.conn, &claims).await?;

    let updated_user = service_update_user_profile(&state.conn, &claims.sub, payload).await?;

    Ok(updated_user)
}
