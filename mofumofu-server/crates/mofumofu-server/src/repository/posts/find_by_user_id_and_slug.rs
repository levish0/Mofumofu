use mofumofu_entity::posts::{Column as PostColumn, Entity as PostEntity, Model as PostModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_find_post_by_user_id_and_slug<C>(
    conn: &C,
    user_id: Uuid,
    slug: String,
) -> Result<Option<PostModel>, Errors>
where
    C: ConnectionTrait,
{
    let post = PostEntity::find()
        .filter(PostColumn::UserId.eq(user_id))
        .filter(PostColumn::Slug.eq(slug))
        .one(conn)
        .await?;

    Ok(post)
}
