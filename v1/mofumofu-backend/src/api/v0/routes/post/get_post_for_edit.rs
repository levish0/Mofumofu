use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::post::request::get_post_for_edit::GetPostForEditRequest;
use crate::dto::post::response::post_edit_info::PostEditInfoResponse;
use crate::service::error::errors::Errors;
use crate::service::post::get_post_for_edit::service_get_post_for_edit;
use crate::service::validator::json_validator::ValidatedJson;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/post/edit",
    request_body = GetPostForEditRequest,
    responses(
        (status = StatusCode::OK, description = "Post retrieved for editing successfully", body = PostEditInfoResponse),
        (status = StatusCode::NOT_FOUND, description = "Post not found"),
        (status = StatusCode::BAD_REQUEST, description = "Invalid input"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Post"
)]
#[axum::debug_handler]
pub async fn get_post_for_edit(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    ValidatedJson(payload): ValidatedJson<GetPostForEditRequest>,
) -> Result<impl IntoResponse, Errors> {
    info!("Received POST request to get post for edit: {:?}", payload);
    let user_uuid = claims.sub.clone();

    let post = service_get_post_for_edit(&state.conn, &payload.slug, &user_uuid).await?;

    Ok(post)
}
