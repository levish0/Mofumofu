use crate::extractors::RequiredSession;
use crate::service::drafts::service_delete_draft;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use mofumofu_dto::drafts::DraftIdPath;
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    delete,
    path = "/v0/drafts/{draft_id}",
    params(DraftIdPath),
    responses(
        (status = 204, description = "Draft deleted successfully"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 404, description = "Not Found - Draft does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Drafts"
)]
pub async fn delete_draft(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<DraftIdPath>,
) -> Result<StatusCode, Errors> {
    service_delete_draft(&state.write_db, session_context.user_id, path.draft_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
