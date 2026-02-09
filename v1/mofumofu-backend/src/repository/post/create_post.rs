use crate::dto::post::request::create_post::CreatePostRequest;
use crate::entity::posts::{ActiveModel as PostActiveModel, Model as PostModel};
use crate::service::error::errors::Errors;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};
use serde_json::Value as JsonValue;
use uuid::Uuid;

pub async fn repository_create_post<C>(
    txn: &C,
    payload: CreatePostRequest,
    user_uuid: &Uuid,
    render_html: Option<String>,
    toc_json: Option<JsonValue>,
) -> Result<PostModel, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_post = PostActiveModel {
        id: Default::default(),
        title: Set(payload.title),
        thumbnail_image: Default::default(),
        summary: Set(payload.summary),
        user_id: Set(*user_uuid),
        content: Set(payload.content),
        created_at: Set(Utc::now()),
        updated_at: Set(Option::from(Utc::now())),
        like_count: Set(0),
        comment_count: Set(0),
        view_count: Set(0),
        slug: Set(payload.slug),
        render: Set(render_html),
        toc: Set(toc_json),
    };

    // Insert the new post
    let created_post = new_post.insert(txn).await?;

    Ok(created_post)
}
