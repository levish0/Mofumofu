use crate::entity::posts::{
    ActiveModel as PostActiveModel, Entity as PostEntity, Model as PostModel,
};
use crate::service::error::errors::Errors;
use sea_orm::{ActiveModelTrait, ActiveValue, ConnectionTrait, EntityTrait, Set, TransactionTrait};
use uuid::Uuid;

pub async fn repository_update_post_thumbnail<C>(
    conn: &C,
    post_id: &Uuid,
    thumbnail_url: Option<String>,
) -> Result<PostModel, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    // Check if post exists
    let existing_post = PostEntity::find_by_id(*post_id)
        .one(conn)
        .await?
        .ok_or(Errors::PostNotFound)?;

    // Update thumbnail image
    let post_active_model = PostActiveModel {
        id: Set(existing_post.id),
        thumbnail_image: match thumbnail_url {
            Some(url) => Set(Some(url)),
            None => Set(None),
        },
        ..Default::default()
    };

    // Execute update
    let updated_post = post_active_model.update(conn).await?;

    Ok(updated_post)
}
