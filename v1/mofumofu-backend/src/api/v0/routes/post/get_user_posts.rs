use crate::dto::post::request::GetUserPostsRequest;
use crate::dto::post::response::UserPostsResponse;
use crate::service::error::errors::Errors;
use crate::service::post::get_user_posts::service_get_user_posts;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/posts/user",
    request_body = GetUserPostsRequest,
    responses(
        (status = StatusCode::OK, description = "User posts retrieved successfully", body = UserPostsResponse),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    tag = "Post"
)]
pub async fn get_user_posts(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<GetUserPostsRequest>,
) -> Result<UserPostsResponse, Errors> {
    info!("Received POST request to get user posts: {:?}", payload);

    let response = service_get_user_posts(&state.conn, &payload.user_handle).await?;

    Ok(response)
}
