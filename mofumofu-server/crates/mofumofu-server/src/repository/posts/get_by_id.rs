use mofumofu_entity::posts::{Entity as PostEntity, Model as PostModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait};
use uuid::Uuid;

pub async fn repository_get_post_by_id<C>(conn: &C, id: Uuid) -> Result<PostModel, Errors>
where
    C: ConnectionTrait,
{
    let post = PostEntity::find_by_id(id).one(conn).await?;
    post.ok_or(Errors::PostNotFound)
}
