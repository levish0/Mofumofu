use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::draft::response::get_drafts::GetDraftsResponse;
use crate::service::auth::require_verified_user;
use crate::service::draft::get_drafts::service_get_drafts;
use crate::service::error::errors::Errors;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/drafts",
    responses(
        (status = 200, description = "Drafts retrieved successfully", body = GetDraftsResponse),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized or email not verified"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Draft"
)]
pub async fn get_drafts(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
) -> Result<GetDraftsResponse, Errors> {
    info!("Received POST request for user drafts");
    let user_uuid = claims.sub.clone();

    require_verified_user(&state.conn, &claims).await?;

    let response = service_get_drafts(&state.conn, &user_uuid).await?;

    Ok(response)
}
