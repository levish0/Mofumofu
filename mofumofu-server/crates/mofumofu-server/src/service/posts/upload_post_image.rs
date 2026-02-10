use crate::connection::r2_conn::R2Client;
use crate::utils::image_processor::image_validator::{
    generate_image_hash, process_image_for_upload, validate_image,
};
use mofumofu_constants::{POST_IMAGE_MAX_SIZE, post_image_key};
use mofumofu_dto::posts::{UploadPostImageRequest, UploadPostImageResponse};
use mofumofu_errors::errors::Errors;
use tracing::{error, info};

pub async fn service_upload_post_image(
    r2_client: &R2Client,
    payload: UploadPostImageRequest,
) -> Result<UploadPostImageResponse, Errors> {
    let image_info = validate_image(&payload.file, POST_IMAGE_MAX_SIZE)?;

    info!(
        "Uploading post image: mime_type={}, size={} bytes",
        image_info.mime_type, image_info.size
    );

    let processed = process_image_for_upload(&payload.file, &image_info.mime_type)?;

    let hash = generate_image_hash(&processed.data);
    let storage_key = post_image_key(&hash, &processed.extension);

    r2_client
        .upload_with_content_type(&storage_key, processed.data, &processed.content_type)
        .await
        .map_err(|e| {
            error!("Failed to upload post image to R2: {}", e);
            Errors::SysInternalError("Failed to upload image to storage".to_string())
        })?;

    let image_url = r2_client.get_r2_public_url(&storage_key);

    info!("Post image uploaded successfully: {}", image_url);

    Ok(UploadPostImageResponse { image_url })
}
