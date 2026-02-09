use crate::dto::post::request::GetPostByUuidRequest;
use crate::dto::post::response::post_info::PostInfoResponse;
use crate::service::error::errors::Errors;
use crate::service::post::get_post_by_uuid::service_get_post_by_uuid;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;

#[utoipa::path(
    post,
    path = "/v0/post/get",
    request_body = GetPostByUuidRequest,
    responses(
        (status = StatusCode::OK, description = "Post retrieved successfully", body = PostInfoResponse),
        (status = StatusCode::NOT_FOUND, description = "Post not found"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid UUID format"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("anonymous_id_cookie" = [])
    ),
    tag = "Post"
)]
pub async fn get_post(
    State(state): State<AppState>,
    ValidatedJson(req_body): ValidatedJson<GetPostByUuidRequest>,
) -> Result<PostInfoResponse, Errors> {
    let post = service_get_post_by_uuid(&state.conn, &state.http_client, &req_body.post_id).await?;

    Ok(post)
}
