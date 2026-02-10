use crate::extractors::RequiredSession;
use crate::service::posts::service_upload_post_image;
use crate::state::AppState;
use axum::extract::State;
use mofumofu_dto::posts::{UploadPostImageRequest, UploadPostImageResponse};
use mofumofu_dto::validator::multipart_validator::ValidatedMultipart;
use mofumofu_errors::errors::Errors;

#[utoipa::path(
    post,
    path = "/v0/posts/images",
    request_body(content = UploadPostImageRequest, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Post image uploaded successfully", body = UploadPostImageResponse),
        (status = 400, description = "Bad Request - Invalid image or validation error"),
        (status = 401, description = "Unauthorized - Invalid or expired session"),
        (status = 413, description = "Payload Too Large - Image exceeds size limit"),
        (status = 500, description = "Internal Server Error")
    ),
    security(
        ("session_id_cookie" = [])
    ),
    tag = "Posts"
)]
pub async fn upload_post_image(
    State(state): State<AppState>,
    RequiredSession(_session_context): RequiredSession,
    ValidatedMultipart(payload): ValidatedMultipart<UploadPostImageRequest>,
) -> Result<UploadPostImageResponse, Errors> {
    service_upload_post_image(&state.r2_client, payload).await
}
