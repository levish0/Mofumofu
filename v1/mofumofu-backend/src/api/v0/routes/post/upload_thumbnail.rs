use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::post::request::thumbnail_image::PostThumbnailForm;
use crate::dto::post::response::ThumbnailUploadResponse;
use crate::service::auth::require_verified_user;
use crate::service::error::errors::Errors;
use crate::service::post::update_post_thumbnail::service_update_post_thumbnail;
use crate::state::AppState;
use axum::Extension;
use axum::extract::{Multipart, State};
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/post/thumbnail",
    request_body(content = PostThumbnailForm, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Thumbnail image uploaded successfully", body = ThumbnailUploadResponse),
        (status = 400, description = "Invalid file or parameters"),
        (status = 401, description = "Unauthorized or email not verified"),
        (status = 403, description = "Not the owner of the post"),
        (status = 404, description = "Post not found"),
        (status = 413, description = "File too large"),
        (status = 422, description = "Unsupported image format"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Post"
)]
pub async fn upload_thumbnail(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    multipart: Multipart,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received thumbnail image upload request by user: {}",
        claims.sub
    );

    require_verified_user(&state.conn, &claims).await?;

    let public_url =
        service_update_post_thumbnail(&state.conn, &state.cloudflare_r2, &claims.sub, multipart)
            .await?;

    Ok(ThumbnailUploadResponse { public_url })
}
