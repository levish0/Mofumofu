use crate::service::posts::service_get_post_by_slug;
use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use mofumofu_dto::posts::{GetPostBySlugRequest, PostResponse};
use mofumofu_dto::validator::query_validator::ValidatedQuery;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    get,
    path = "/v0/posts/by-slug",
    params(GetPostBySlugRequest),
    responses(
        (status = 200, description = "Post found", body = PostResponse),
        (status = 400, description = "Bad request - Invalid query parameters or validation error"),
        (status = 404, description = "Not Found - User or post does not exist"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Posts"
)]
pub async fn get_post_by_slug(
    State(state): State<AppState>,
    ValidatedQuery(payload): ValidatedQuery<GetPostBySlugRequest>,
) -> Result<Json<PostResponse>, Errors> {
    let response = service_get_post_by_slug(&state.read_db, payload.handle, payload.slug).await?;
    Ok(Json(response))
}
