use crate::dto::comment::request::get_comments::GetCommentsRequest;
use crate::dto::comment::response::get_comments::GetCommentsResponse;
use crate::service::comment::get_comments::service_get_comments;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/comment/list",
    request_body = GetCommentsRequest,
    responses(
        (status = 200, description = "Comments retrieved successfully", body = GetCommentsResponse),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Comment"
)]
pub async fn get_comments(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<GetCommentsRequest>,
) -> Result<GetCommentsResponse, Errors> {
    info!("Received request to get comments: {:?}", payload);

    let response = service_get_comments(&state.conn, payload).await?;

    Ok(response)
}
