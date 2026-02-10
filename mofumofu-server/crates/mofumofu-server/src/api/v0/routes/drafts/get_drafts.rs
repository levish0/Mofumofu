use crate::extractors::RequiredSession;
use crate::service::drafts::service_get_drafts;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::drafts::DraftListResponse;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/drafts",
    responses(
        (status = 200, description = "Drafts retrieved successfully", body = DraftListResponse),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Drafts"
)]
pub async fn get_drafts(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
) -> Result<DraftListResponse, Errors> {
    service_get_drafts(&state.read_db, session_context.user_id).await
}
