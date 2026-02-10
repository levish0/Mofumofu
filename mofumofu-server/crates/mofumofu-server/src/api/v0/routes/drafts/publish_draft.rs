use crate::extractors::RequiredSession;
use crate::service::drafts::service_publish_draft;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use mofumofu_dto::drafts::{DraftIdPath, PublishDraftRequest};
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/drafts/{draft_id}/publish",
    params(DraftIdPath),
    request_body = PublishDraftRequest,
    responses(
        (status = 201, description = "Draft published as post successfully", body = mofumofu_dto::posts::PostResponse),
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
pub async fn publish_draft(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<DraftIdPath>,
    ValidatedJson(payload): ValidatedJson<PublishDraftRequest>,
) -> Result<impl IntoResponse, Errors> {
    let response = service_publish_draft(
        &state.write_db,
        &state.http_client,
        session_context.user_id,
        path.draft_id,
        payload,
    )
    .await?;
    Ok((StatusCode::CREATED, Json(response)))
}
