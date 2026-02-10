use crate::repository::user::get_by_handle::repository_get_user_by_handle;
use crate::utils::r2_url::build_r2_public_url;
use mofumofu_dto::user::PublicUserProfile;
use mofumofu_errors::errors::ServiceResult;
use sea_orm::DatabaseConnection;

pub async fn service_get_user_profile_by_handle(
    conn: &DatabaseConnection,
    handle: &str,
) -> ServiceResult<PublicUserProfile> {
    let user = repository_get_user_by_handle(conn, handle.to_string()).await?;

    let profile = PublicUserProfile {
        id: user.id,
        handle: user.handle,
        display_name: user.display_name,
        bio: user.bio,
        profile_image: user.profile_image.as_deref().map(build_r2_public_url),
        banner_image: user.banner_image.as_deref().map(build_r2_public_url),
        is_verified: user.verified_at.is_some(),
        created_at: user.created_at,
    };

    Ok(profile)
}
