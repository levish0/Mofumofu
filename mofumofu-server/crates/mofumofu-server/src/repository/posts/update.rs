use mofumofu_entity::posts::{
    ActiveModel as PostActiveModel, Entity as PostEntity, Model as PostModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::prelude::{DateTimeUtc, Json};
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel, Set};
use uuid::Uuid;

#[derive(Default)]
pub struct PostUpdateParams {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub thumbnail_image: Option<Option<String>>,
    pub summary: Option<Option<String>>,
    pub content: Option<String>,
    pub render: Option<Option<String>>,
    pub toc: Option<Option<Json>>,
    pub published_at: Option<Option<DateTimeUtc>>,
}

pub async fn repository_update_post<C>(
    conn: &C,
    post_id: Uuid,
    params: PostUpdateParams,
) -> Result<PostModel, Errors>
where
    C: ConnectionTrait,
{
    let post = PostEntity::find_by_id(post_id)
        .one(conn)
        .await?
        .ok_or(Errors::PostNotFound)?;

    let mut post_active: PostActiveModel = post.into_active_model();

    if let Some(title) = params.title {
        post_active.title = Set(title);
    }
    if let Some(slug) = params.slug {
        post_active.slug = Set(slug);
    }
    if let Some(thumbnail_image) = params.thumbnail_image {
        post_active.thumbnail_image = Set(thumbnail_image);
    }
    if let Some(summary) = params.summary {
        post_active.summary = Set(summary);
    }
    if let Some(content) = params.content {
        post_active.content = Set(content);
    }
    if let Some(render) = params.render {
        post_active.render = Set(render);
    }
    if let Some(toc) = params.toc {
        post_active.toc = Set(toc);
    }
    if let Some(published_at) = params.published_at {
        post_active.published_at = Set(published_at);
    }

    let updated_post = post_active.update(conn).await?;
    Ok(updated_post)
}
