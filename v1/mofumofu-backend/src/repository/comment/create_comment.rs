use crate::entity::comments::{ActiveModel as CommentActiveModel, Model as CommentModel};
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set};
use uuid::Uuid;

pub async fn repository_create_comment<C>(
    conn: &C,
    user_id: Uuid,
    post_id: Uuid,
    content: &str,
    parent_id: Option<Uuid>,
) -> Result<CommentModel, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let new_comment = CommentActiveModel {
        id: Default::default(),
        content: Set(content.to_string()),
        post_id: Set(post_id),
        user_id: Set(user_id),
        parent_id: Set(parent_id),
        created_at: Default::default(),
        updated_at: Set(None),
        is_deleted: Set(false),
        like_count: Set(0),
        reply_count: Set(0),
    };

    let created_comment = new_comment.insert(conn).await?;
    Ok(created_comment)
}
