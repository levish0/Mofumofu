use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::follow::internal::delete::DeleteFollow;
use crate::dto::follow::request::delete::DeleteFollowRequest;
use crate::service::error::errors::Errors;
use crate::service::follow::delete_follow::service_delete_follow_by_handle;
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

// 팔로우 취소
#[utoipa::path(
    delete,
    path = "/v0/follow",
    request_body = DeleteFollowRequest,
    responses(
        (status = StatusCode::NO_CONTENT, description = "Successfully unfollowed user"),
        (status = StatusCode::BAD_REQUEST, description = "Bad request"),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Follow"
)]
pub async fn api_delete_follow(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    Json(payload): Json<DeleteFollowRequest>,
) -> Result<impl IntoResponse, Errors> {
    let user_uuid = claims.sub.clone();

    service_delete_follow_by_handle(
        &state.conn,
        DeleteFollow {
            follower_id: user_uuid,
            followee_handle: payload.followee_handle,
        },
    )
    .await?;

    Ok(StatusCode::NO_CONTENT)
}
