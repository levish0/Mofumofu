use crate::entity::posts;
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use tracing::{error, info};
use uuid::Uuid;

pub async fn repository_get_post_by_uuid<C>(
    conn: &C,
    post_id: &Uuid,
) -> Result<posts::Model, Errors>
where
    C: ConnectionTrait,
{
    info!("Fetching post by ID: {}", post_id);

    let post = posts::Entity::find()
        .filter(posts::Column::Id.eq(*post_id))
        .one(conn)
        .await
        .map_err(|e| {
            error!(
                "Database error while fetching post by ID {}: {}",
                post_id, e
            );
            Errors::SysInternalError("Failed to fetch post".to_string())
        })?;

    match post {
        Some(post) => {
            info!("Post found: {}", post_id);
            Ok(post)
        }
        None => {
            info!("Post not found: {}", post_id);
            Err(Errors::PostNotFound)
        }
    }
}
