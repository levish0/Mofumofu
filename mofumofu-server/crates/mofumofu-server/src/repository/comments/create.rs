use mofumofu_entity::comments::{ActiveModel as CommentActiveModel, Model as CommentModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_comment<C>(
    conn: &C,
    post_id: Uuid,
    user_id: Uuid,
    parent_id: Option<Uuid>,
    depth: i32,
    content: String,
) -> Result<CommentModel, Errors>
where
    C: ConnectionTrait,
{
    let new_comment = CommentActiveModel {
        id: Default::default(),
        post_id: Set(post_id),
        user_id: Set(user_id),
        parent_id: Set(parent_id),
        depth: Set(depth),
        content: Set(content),
        like_count: Set(0),
        deleted_at: Set(None),
        created_at: Default::default(),
        updated_at: Default::default(),
    };

    let comment = new_comment.insert(conn).await?;
    Ok(comment)
}
