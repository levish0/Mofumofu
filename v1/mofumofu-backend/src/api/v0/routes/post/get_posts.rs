use crate::dto::post::request::GetPostsRequest;
use crate::dto::post::response::GetPostsResponse;
use crate::service::error::errors::Errors;
use crate::service::post::get_posts::service_get_posts;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/posts",
    request_body = GetPostsRequest,
    responses(
        (status = StatusCode::OK, description = "Posts retrieved successfully", body = GetPostsResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Post"
)]
pub async fn get_posts(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<GetPostsRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received POST request to get posts: {:?}", payload);

    let response = service_get_posts(&state.conn, payload).await?;

    Ok(response)
}
