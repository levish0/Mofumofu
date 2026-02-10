use crate::extractors::RequiredSession;
use crate::service::posts::service_create_post;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use mofumofu_dto::posts::CreatePostRequest;
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/posts",
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "Post created successfully", body = mofumofu_dto::posts::PostResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Posts"
)]
pub async fn create_post(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedJson(payload): ValidatedJson<CreatePostRequest>,
) -> Result<impl IntoResponse, Errors> {
    let response = service_create_post(
        &state.write_db,
        &state.http_client,
        session_context.user_id,
        payload,
    )
    .await?;
    Ok((StatusCode::CREATED, Json(response)))
}
