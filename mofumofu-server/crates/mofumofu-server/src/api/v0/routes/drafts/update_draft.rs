use crate::extractors::RequiredSession;
use crate::service::drafts::service_update_draft;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use mofumofu_dto::drafts::{DraftIdPath, DraftResponse, UpdateDraftRequest};
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    patch,
    path = "/v0/drafts/{draft_id}",
    params(DraftIdPath),
    request_body = UpdateDraftRequest,
    responses(
        (status = 200, description = "Draft updated successfully", body = DraftResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 404, description = "Not Found - Draft does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Drafts"
)]
pub async fn update_draft(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<DraftIdPath>,
    ValidatedJson(payload): ValidatedJson<UpdateDraftRequest>,
) -> Result<Json<DraftResponse>, Errors> {
    let response = service_update_draft(
        &state.write_db,
        session_context.user_id,
        path.draft_id,
        payload,
    )
    .await?;
    Ok(Json(response))
}
