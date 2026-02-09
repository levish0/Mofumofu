use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::comment::request::create_comment::CreateCommentRequest;
use crate::dto::comment::response::create_comment::CreateCommentResponse;
use crate::service::comment::create_comment::service_create_comment;
use crate::service::error::errors::Errors;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/comment",
    request_body = CreateCommentRequest,
    responses(
        (status = 201, description = "Comment created successfully", body = CreateCommentResponse),
        (status = StatusCode::NOT_FOUND, description = "Post not found or Parent comment not found"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid parent comment or Cannot reply to deleted comment"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Comment"
)]
pub async fn create_comment(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<CreateCommentRequest>,
) -> Result<CreateCommentResponse, Errors> {
    info!("Received request to create comment: {:?}", payload);
    let user_uuid = claims.sub.clone();

    let response = service_create_comment(&state.conn, &user_uuid, payload).await?;

    Ok(response)
}
