use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::draft::request::get_draft::GetDraftRequest;
use crate::dto::draft::response::draft_info::DraftInfo;
use crate::service::auth::require_verified_user;
use crate::service::draft::get_draft::service_get_draft;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/draft/get",
    request_body = GetDraftRequest,
    responses(
        (status = 200, description = "Draft retrieved successfully", body = DraftInfo),
        (status = StatusCode::NOT_FOUND, description = "Draft not found: draft:not_found"),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized or email not verified"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Draft"
)]
#[axum::debug_handler]
pub async fn get_draft(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<GetDraftRequest>,
) -> Result<DraftInfo, Errors> {
    info!("Received POST request for draft: {}", payload.draft_id);
    let user_uuid = claims.sub.clone();

    require_verified_user(&state.conn, &claims).await?;

    let response = service_get_draft(&state.conn, &payload.draft_id, &user_uuid).await?;

    Ok(response)
}
