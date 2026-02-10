use crate::service::posts::service_get_post;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use mofumofu_dto::posts::PostIdPath;
use mofumofu_dto::posts::PostResponse;
use mofumofu_dto::validator::path_validator::ValidatedPath;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/posts/{post_id}",
    params(PostIdPath),
    responses(
        (status = 200, description = "Post retrieved successfully", body = PostResponse),
        (status = 400, description = "Bad request - Invalid path parameter"),
        (status = 404, description = "Not Found - Post does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Posts"
)]
pub async fn get_post(
    State(state): State<AppState>,
    ValidatedPath(path): ValidatedPath<PostIdPath>,
) -> Result<Json<PostResponse>, Errors> {
    let response = service_get_post(&state.read_db, &state.redis_cache, path.post_id).await?;
    Ok(Json(response))
}
