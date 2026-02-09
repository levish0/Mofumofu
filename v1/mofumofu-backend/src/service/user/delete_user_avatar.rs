use crate::connection::cloudflare_r2::R2Client;
use crate::dto::user::internal::update_user::UpdateUserFields;
use crate::repository::user::get_user_by_uuid::repository_get_user_by_uuid;
use crate::repository::user::update_user::repository_update_user;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::{ConnectionTrait, TransactionTrait};
use tracing::{error, info, warn};
use uuid::Uuid;

pub async fn service_delete_user_avatar<C>(
    conn: &C,
    r2_client: &R2Client,
    user_uuid: &Uuid,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    info!("Processing avatar image delete for user: {}", user_uuid);

    // UUID로 사용자 정보 조회
    let user = repository_get_user_by_uuid(conn, user_uuid).await?;

    // 기존 아바타가 있는 경우 R2에서 삭제
    if let Some(existing_image_url) = &user.profile_image {
        if !existing_image_url.is_empty() {
            // URL에서 키 추출 (profiles/{handle}/avatar/{filename} 형태)
            // URL 예: https://domain.com/profiles/user123/avatar/avatar_hash.jpg
            let url_parts: Vec<&str> = existing_image_url.split('/').collect();
            if url_parts.len() >= 4 {
                // "profiles/{handle}/avatar/{filename}" 부분 추출
                let key = url_parts[url_parts.len() - 4..].join("/");
                if let Err(e) = r2_client.delete(&key).await {
                    warn!("Failed to delete avatar from R2: {}", e);
                }
            }
        }
    }

    // 데이터베이스에서 profile_image 필드를 NULL로 설정
    let update_fields = UpdateUserFields {
        profile_image: Some(None),
        ..Default::default()
    };

    repository_update_user(conn, user_uuid, update_fields)
        .await
        .map_err(|e| {
            error!("Failed to clear user profile image in database: {:?}", e);
            Errors::SysInternalError("Failed to clear user profile".to_string())
        })?;

    info!("Avatar image deleted successfully for user: {}", user_uuid);

    Ok(())
}
