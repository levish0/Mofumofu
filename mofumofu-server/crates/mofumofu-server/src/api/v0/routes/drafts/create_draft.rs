use crate::extractors::RequiredSession;
use crate::service::drafts::service_create_draft;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/drafts",
    responses(
        (status = 201, description = "Draft created successfully", body = mofumofu_dto::drafts::DraftResponse),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Drafts"
)]
pub async fn create_draft(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
) -> Result<impl IntoResponse, Errors> {
    let response = service_create_draft(&state.write_db, session_context.user_id).await?;
    Ok((StatusCode::CREATED, Json(response)))
}
