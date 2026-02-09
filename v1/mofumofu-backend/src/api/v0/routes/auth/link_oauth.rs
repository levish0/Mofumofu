use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::auth::request::link_oauth::LinkOAuthRequest;
use crate::service::auth::service_link_oauth;
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
    path = "/v0/auth/link_oauth",
    request_body = LinkOAuthRequest,
    responses(
        (status = 200, description = "OAuth account linked successfully"),
        (status = 400, description = "OAuth errors: oauth:invalid_auth_url, oauth:invalid_token_url, oauth:invalid_redirect_url, oauth:token_exchange_failed, oauth:user_info_fetch_failed, oauth:invalid_image_url"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
        (status = 409, description = "OAuth account already linked: oauth:account_already_linked"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error or oauth:user_info_parse_failed")
    ),
    tag = "Auth",
    security(("bearer_auth" = []))
)]
pub async fn link_oauth(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<LinkOAuthRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received POST request to link OAuth for user: {}",
        claims.sub
    );

    service_link_oauth(&state, claims.sub, payload).await?;

    Ok(StatusCode::OK)
}
