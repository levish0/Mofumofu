use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::follow::internal::create::CreateFollow;
use crate::dto::follow::request::create::CreateFollowRequest;
use crate::service::error::errors::Errors;
use crate::service::follow::create_follow::service_create_follow_by_handle;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

// 팔로우 생성
#[utoipa::path(
    post,
    path = "/v0/follow",
    request_body = CreateFollowRequest,
    responses(
        (status = 204, description = "Successfully followed user"),
        (status = 400, description = "Bad request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "User not found"),
        (status = 409, description = "Already following this user"),
        (status = 422, description = "Cannot follow yourself"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Follow"
)]
pub async fn api_create_follow(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Json(payload): Json<CreateFollowRequest>,
) -> Result<impl IntoResponse, Errors> {
    let user_uuid = claims.sub.clone();

    service_create_follow_by_handle(
        &state.conn,
        CreateFollow {
            follower_id: user_uuid,
            followee_handle: payload.followee_handle,
        },
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
