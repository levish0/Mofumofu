use crate::connection::cloudflare_r2::R2Client;
use crate::dto::user::internal::update_user::UpdateUserFields;
use crate::repository::user::get_user_by_uuid::repository_get_user_by_uuid;
use crate::repository::user::update_user::repository_update_user;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::utils::image_validator::{generate_image_hash, process_image_for_upload};
use axum::extract::Multipart;
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::{error, info, warn};
use uuid::Uuid;

pub async fn service_update_user_banner<C>(
    conn: &C,
    r2_client: &R2Client,
    user_uuid: &Uuid,
    mut multipart: Multipart,
) -> ServiceResult<String>
where
    C: ConnectionTrait + TransactionTrait,
{
    info!("Processing banner image upload for user: {}", user_uuid);

    // UUID로 사용자 정보 조회
    let user = repository_get_user_by_uuid(conn, user_uuid).await?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| Errors::FileReadError(format!("Failed to read multipart field: {}", e)))?
    {
        if field.name() == Some("file") {
            let data = field
                .bytes()
                .await
                .map_err(|e| Errors::FileReadError(format!("Failed to read file data: {}", e)))?;

            // Process and compress image (8MB limit for banner)
            const MAX_BANNER_SIZE: usize = 8 * 1024 * 1024;
            let max_dimensions = Some((1600, 400)); // Max dimensions for banner
            let (processed_data, content_type, extension) = process_image_for_upload(
                &data,
                MAX_BANNER_SIZE,
                true, // Convert to WebP for better compression
                max_dimensions,
            )?;

            // Generate hash-based filename using processed data
            let hash = generate_image_hash(&processed_data);
            let filename = format!("banner_{}.{}", hash, extension);

            info!(
                "Processing banner image upload: user_uuid={}, filename={}, content_type={}, original_size={} bytes, processed_size={} bytes",
                user_uuid,
                filename,
                content_type,
                data.len(),
                processed_data.len()
            );

            // Delete existing banner if exists
            if let Some(existing_image_url) = &user.banner_image {
                if !existing_image_url.is_empty() {
                    // Extract key from URL and delete from R2
                    let url_parts: Vec<&str> = existing_image_url.split('/').collect();
                    if url_parts.len() >= 4 {
                        let key = url_parts[url_parts.len() - 4..].join("/");
                        if let Err(e) = r2_client.delete(&key).await {
                            warn!("Failed to delete existing banner from R2: {}", e);
                        }
                    }
                }
            }

            // Upload to R2
            let r2_key = format!("profiles/{}/banner/{}", user.handle, filename);
            r2_client
                .upload_with_content_type(&r2_key, processed_data, &content_type)
                .await
                .map_err(|e| {
                    error!("Failed to upload banner to R2: {}", e);
                    Errors::SysInternalError("Failed to upload banner image".to_string())
                })?;

            // Get public URL
            let public_url = r2_client.get_r2_public_url(&r2_key);

            // Update user banner image in database
            let update_fields = UpdateUserFields {
                banner_image: Some(Some(public_url.clone())),
                ..Default::default()
            };

            repository_update_user(conn, user_uuid, update_fields)
                .await
                .map_err(|e| {
                    error!("Failed to update user banner image in database: {:?}", e);
                    Errors::SysInternalError("Failed to update user profile".to_string())
                })?;

            info!("Banner image uploaded successfully: {}", public_url);
            return Ok(public_url);
        }
    }

    Err(Errors::FileNotFound)
}
