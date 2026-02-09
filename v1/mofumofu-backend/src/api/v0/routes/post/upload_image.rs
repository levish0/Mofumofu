use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::post::request::image_upload::ImageUploadForm;
use crate::dto::post::response::ImageUploadResponse;
use crate::service::auth::require_verified_user;
use crate::service::error::errors::Errors;
use crate::service::post::upload_image::service_upload_image;
use crate::state::AppState;
use axum::Extension;
use axum::extract::{Multipart, State};
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/post/image",
    request_body(content = ImageUploadForm, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Image uploaded successfully", body = ImageUploadResponse),
        (status = 400, description = "File errors: file:not_found, file:read_error"),
        (status = 401, description = "Unauthorized or email not verified"),
        (status = 413, description = "File too large"),
        (status = 422, description = "Unsupported image format"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Post"
)]
pub async fn upload_image(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    multipart: Multipart,
) -> Result<impl IntoResponse, Errors> {
    info!("Received image upload request by user: {}", claims.sub);

    require_verified_user(&state.conn, &claims).await?;

    let filename = service_upload_image(&state.cloudflare_r2, &claims.sub, multipart).await?;
    let public_url = state
        .cloudflare_r2
        .get_r2_public_url(&format!("post-images/{}", filename));

    Ok(ImageUploadResponse { public_url })
}
