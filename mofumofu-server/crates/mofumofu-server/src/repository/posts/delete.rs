use mofumofu_entity::posts::Entity as PostEntity;
use mofumofu_errors::errors::Errors;
use sea_orm::{ConnectionTrait, DeleteResult, EntityTrait, ModelTrait};
use uuid::Uuid;

pub async fn repository_delete_post<C>(conn: &C, post_id: Uuid) -> Result<DeleteResult, Errors>
where
    C: ConnectionTrait,
{
    let post = PostEntity::find_by_id(post_id)
        .one(conn)
        .await?
        .ok_or(Errors::PostNotFound)?;

    let result = post.delete(conn).await?;
    Ok(result)
}
