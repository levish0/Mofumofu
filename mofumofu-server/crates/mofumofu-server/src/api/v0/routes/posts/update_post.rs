use crate::extractors::RequiredSession;
use crate::service::posts::service_update_post;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use mofumofu_dto::posts::{PostIdPath, PostResponse, UpdatePostRequest};
use mofumofu_dto::validator::json_validator::ValidatedJson;
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    patch,
    path = "/v0/posts/{post_id}",
    params(PostIdPath),
    request_body = UpdatePostRequest,
    responses(
        (status = 200, description = "Post updated successfully", body = PostResponse),
        (status = 400, description = "Bad request - Invalid JSON or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 403, description = "Forbidden - Not the post author"),
        (status = 404, description = "Not Found - Post does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Posts"
)]
pub async fn update_post(
    State(state): State<AppState>,
    RequiredSession(session_context): RequiredSession,
    ValidatedPath(path): ValidatedPath<PostIdPath>,
    ValidatedJson(payload): ValidatedJson<UpdatePostRequest>,
) -> Result<Json<PostResponse>, Errors> {
    let response = service_update_post(
        &state.write_db,
        &state.http_client,
        session_context.user_id,
        path.post_id,
        payload,
    )
    .await?;
    Ok(Json(response))
}
