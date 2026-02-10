use crate::extractors::RequiredSession;
use crate::service::drafts::service_get_draft;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use mofumofu_dto::drafts::{DraftIdPath, DraftResponse};
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/drafts/{draft_id}",
    params(DraftIdPath),
    responses(
        (status = 200, description = "Draft retrieved successfully", body = DraftResponse),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 404, description = "Not Found - Draft does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Drafts"
)]
pub async fn get_draft(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<DraftIdPath>,
) -> Result<Json<DraftResponse>, Errors> {
    let response =
        service_get_draft(&state.read_db, session_context.user_id, path.draft_id).await?;
    Ok(Json(response))
}
