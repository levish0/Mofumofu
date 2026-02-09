use crate::entity::posts;
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use tracing::{error, info};
use uuid::Uuid;

pub async fn repository_get_post_by_user_and_slug<C>(
    conn: &C,
    user_id: &Uuid,
    slug: &str,
) -> Result<posts::Model, Errors>
where
    C: ConnectionTrait,
{
    info!("Fetching post by user_id: {} and slug: {}", user_id, slug);

    let post = posts::Entity::find()
        .filter(posts::Column::UserId.eq(*user_id))
        .filter(posts::Column::Slug.eq(slug))
        .one(conn)
        .await
        .map_err(|e| {
            error!(
                "Database error while fetching post by user_id {} and slug {}: {}",
                user_id, slug, e
            );
            Errors::SysInternalError("Failed to fetch post".to_string())
        })?;

    match post {
        Some(post) => {
            info!("Post found: user_id={}, slug={}", user_id, slug);
            Ok(post)
        }
        None => {
            info!("Post not found: user_id={}, slug={}", user_id, slug);
            Err(Errors::PostNotFound)
        }
    }
}
