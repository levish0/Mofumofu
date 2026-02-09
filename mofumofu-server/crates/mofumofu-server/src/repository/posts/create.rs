use mofumofu_entity::posts::{ActiveModel as PostActiveModel, Model as PostModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_post<C>(
    conn: &C,
    user_id: Uuid,
    title: String,
    slug: String,
    content: String,
    summary: Option<String>,
    thumbnail_image: Option<String>,
) -> Result<PostModel, Errors>
where
    C: ConnectionTrait,
{
    let new_post = PostActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        title: Set(title),
        slug: Set(slug),
        thumbnail_image: Set(thumbnail_image),
        summary: Set(summary),
        content: Set(content),
        render: Set(None),
        toc: Set(None),
        like_count: Set(0),
        comment_count: Set(0),
        view_count: Set(0),
        published_at: Set(None),
        created_at: Default::default(),
        updated_at: Default::default(),
    };

    let post = new_post.insert(conn).await?;
    Ok(post)
}
