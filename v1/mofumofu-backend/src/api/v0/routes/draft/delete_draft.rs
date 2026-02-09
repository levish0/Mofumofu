use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::draft::request::delete_draft::DeleteDraftRequest;
use crate::service::auth::require_verified_user;
use crate::service::draft::delete_draft::service_delete_draft;
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
    path = "/v0/draft/delete",
    request_body = DeleteDraftRequest,
    responses(
        (status = 204, description = "Draft deleted successfully"),
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
pub async fn delete_draft(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<DeleteDraftRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received POST request to delete draft: {}",
        payload.draft_id
    );
    let user_uuid = claims.sub.clone();

    require_verified_user(&state.conn, &claims).await?;

    service_delete_draft(&state.conn, &payload.draft_id, &user_uuid).await?;

    Ok(StatusCode::NO_CONTENT)
}
