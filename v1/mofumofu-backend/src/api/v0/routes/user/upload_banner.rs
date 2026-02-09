use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::user::request::banner_image::ProfileBannerForm;
use crate::dto::user::response::image_upload::ImageUploadResponse;
use crate::service::auth::require_verified_user;
use crate::service::error::errors::Errors;
use crate::service::user::update_user_banner::service_update_user_banner;
use crate::state::AppState;
use axum::Extension;
use axum::extract::{Multipart, State};
use axum::response::IntoResponse;
use tracing::info;

#[utoipa::path(
    post,
    path = "/v0/user/profile/banner",
    request_body(content = ProfileBannerForm, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Banner image upload queued successfully", body = ImageUploadResponse),
        (status = 400, description = "File errors: file:not_found, file:read_error"),
        (status = 401, description = "Unauthorized or email not verified"),
        (status = 413, description = "File too large"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "User"
)]
pub async fn upload_banner(
    State(state): State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
    multipart: Multipart,
) -> Result<impl IntoResponse, Errors> {
    info!(
        "Received banner image upload request for user: {}",
        claims.sub
    );

    require_verified_user(&state.conn, &claims).await?;

    let public_url =
        service_update_user_banner(&state.conn, &state.cloudflare_r2, &claims.sub, multipart)
            .await?;

    Ok(ImageUploadResponse { public_url })
}
