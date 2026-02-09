use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::draft::request::update_draft::UpdateDraftRequest;
use axum::response::IntoResponse;
use crate::service::auth::require_verified_user;
use crate::service::draft::update_draft::service_update_draft;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/draft/update",
    request_body = UpdateDraftRequest,
    responses(
        (status = 204, description = "Draft updated successfully"),
        (status = StatusCode::NOT_FOUND, description = "Draft not found: draft:not_found"),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized or email not verified"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Draft"
)]
pub async fn update_draft(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<UpdateDraftRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received POST request to update draft");
    let user_uuid = claims.sub.clone();

    require_verified_user(&state.conn, &claims).await?;

    service_update_draft(&state.conn, payload, &user_uuid).await?;

    Ok(StatusCode::NO_CONTENT)
}
