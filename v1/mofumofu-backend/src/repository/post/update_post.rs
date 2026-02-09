use crate::dto::post::request::update_post::UpdatePostRequest;
use crate::entity::posts::{
    ActiveModel as PostActiveModel, Column, Entity as PostEntity, Model as PostModel,
};
use crate::service::error::errors::Errors;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use serde_json::Value as JsonValue;
use uuid::Uuid;

pub async fn repository_update_post<C>(
    conn: &C,
    payload: UpdatePostRequest,
    user_uuid: &Uuid,
    render_html: Option<String>,
    toc_json: Option<JsonValue>,
) -> Result<PostModel, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let post = PostEntity::find()
        .filter(Column::Id.eq(payload.post_id))
        .filter(Column::UserId.eq(*user_uuid))
        .one(conn)
        .await?
        .ok_or(Errors::PostNotFound)?;

    let mut active_post: PostActiveModel = post.into();

    if let Some(title) = payload.title {
        active_post.title = Set(title);
    }

    if let Some(summary) = payload.summary {
        active_post.summary = Set(summary);
    }

    if let Some(content) = payload.content {
        active_post.content = Set(content);

        // render_html과 toc_json이 제공된 경우 업데이트
        if let Some(html) = render_html {
            active_post.render = Set(Some(html));
        }
        if let Some(toc) = toc_json {
            active_post.toc = Set(Some(toc));
        }
    }

    if let Some(new_slug) = payload.new_slug {
        active_post.slug = Set(new_slug);
    }

    active_post.updated_at = Set(Some(Utc::now()));

    let updated_post = active_post.update(conn).await?;

    Ok(updated_post)
}
