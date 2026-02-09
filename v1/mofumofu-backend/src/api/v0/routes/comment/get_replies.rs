use crate::dto::comment::request::get_comments::GetRepliesRequest;
use crate::dto::comment::response::get_comments::GetRepliesResponse;
use crate::service::comment::get_replies::service_get_replies;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/comment/replies",
    request_body = GetRepliesRequest,
    responses(
        (status = 200, description = "Replies retrieved successfully", body = GetRepliesResponse),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "Comment"
)]
pub async fn get_replies(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<GetRepliesRequest>,
) -> Result<GetRepliesResponse, Errors> {
    info!("Received request to get replies: {:?}", payload);

    let response = service_get_replies(&state.conn, payload).await?;

    Ok(response)
}
