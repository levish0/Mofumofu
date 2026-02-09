use crate::connection::cloudflare_r2::R2Client;
use crate::repository::post::get_post_by_uuid::repository_get_post_by_uuid;
use crate::repository::post::update_post_thumbnail::repository_update_post_thumbnail;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::utils::image_validator::{generate_image_hash, process_image_for_upload};
use axum::extract::Multipart;
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::{error, info, warn};
use uuid::Uuid;

pub async fn service_update_post_thumbnail<C>(
    conn: &C,
    r2_client: &R2Client,
    user_uuid: &Uuid,
    mut multipart: Multipart,
) -> ServiceResult<String>
where
    C: ConnectionTrait + TransactionTrait,
{
    info!("Processing thumbnail image upload by user: {}", user_uuid);

    let mut file_data: Option<Vec<u8>> = None;
    let mut content_type: Option<String> = None;
    let mut post_id: Option<Uuid> = None;

    // multipart 데이터 파싱
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        error!("Failed to read multipart field: {}", e);
        Errors::FileReadError("Invalid multipart data".to_string())
    })? {
        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "file" => {
                content_type = field.content_type().map(|ct| ct.to_string());
                let data = field.bytes().await.map_err(|e| {
                    error!("Failed to read image data: {}", e);
                    Errors::FileReadError("Failed to read image data".to_string())
                })?;

                file_data = Some(data.to_vec());
            }
            "post_id" => {
                let text = field.text().await.map_err(|e| {
                    error!("Failed to read post_id field: {}", e);
                    Errors::BadRequestError("Failed to read post_id".to_string())
                })?;
                post_id =
                    Some(Uuid::parse_str(&text).map_err(|_| {
                        Errors::BadRequestError("Invalid post_id format".to_string())
                    })?);
            }
            _ => {
                warn!("Unknown field in multipart: {}", field_name);
            }
        }
    }

    // post_id 필드 검증
    let post_id = post_id.ok_or_else(|| {
        error!("No post_id provided in multipart data");
        Errors::BadRequestError("Post ID is required".to_string())
    })?;

    info!(
        "Processing thumbnail image upload for post ID: {} by user: {}",
        post_id, user_uuid
    );

    // 포스트 존재 확인 및 작성자 권한 검증
    let post = repository_get_post_by_uuid(conn, &post_id).await?;

    // 작성자 권한 검증
    if post.user_id != *user_uuid {
        return Err(Errors::ForbiddenError(
            "Not the owner of this post".to_string(),
        ));
    }

    // 필수 필드 검증
    let file_data = file_data.ok_or_else(|| {
        error!("No image file provided in multipart data");
        Errors::BadRequestError("Image file is required".to_string())
    })?;

    // Process and compress image (4MB limit for thumbnails)
    const MAX_THUMBNAIL_SIZE: usize = 4 * 1024 * 1024;
    let max_dimensions = Some((800, 450)); // Max dimensions for thumbnail
    let (processed_data, content_type, extension) = process_image_for_upload(
        &file_data,
        MAX_THUMBNAIL_SIZE,
        true, // Convert to WebP for better compression
        max_dimensions,
    )?;

    // Generate hash-based filename using processed data
    let hash = generate_image_hash(&processed_data);
    let filename = format!("thumbnail_{}.{}", hash, extension);

    info!(
        "Processing thumbnail image upload: post_id={}, user_uuid={}, filename={}, content_type={}, original_size={} bytes, processed_size={} bytes",
        post_id,
        user_uuid,
        filename,
        content_type,
        file_data.len(),
        processed_data.len()
    );

    // Delete existing thumbnail if exists
    if let Some(existing_thumbnail_url) = &post.thumbnail_image {
        if !existing_thumbnail_url.is_empty() {
            // Extract key from URL and delete from R2
            let key = format!("posts/{}/thumbnail/{}", post.id, filename);
            if let Err(e) = r2_client.delete(&key).await {
                warn!("Failed to delete existing thumbnail from R2: {}", e);
            }
        }
    }

    // Upload to R2
    let r2_key = format!("posts/{}/thumbnail/{}", post.id, filename);
    r2_client
        .upload_with_content_type(&r2_key, processed_data, &content_type)
        .await
        .map_err(|e| {
            error!("Failed to upload thumbnail to R2: {}", e);
            Errors::SysInternalError("Failed to upload thumbnail image".to_string())
        })?;

    // Get public URL
    let public_url = r2_client.get_r2_public_url(&r2_key);

    // Update post thumbnail in database
    repository_update_post_thumbnail(conn, &post.id, Some(public_url.clone()))
        .await
        .map_err(|e| {
            error!("Failed to update post thumbnail in database: {:?}", e);
            Errors::SysInternalError("Failed to update post thumbnail".to_string())
        })?;

    info!("Thumbnail image uploaded successfully: {}", public_url);
    Ok(public_url)
}
