use crate::entity::comments::{
    ActiveModel as CommentActiveModel, Entity as CommentEntity, Model as CommentModel,
};
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, Set};
use uuid::Uuid;

pub async fn repository_update_comment<C>(
    conn: &C,
    comment_id: Uuid,
    content: &str,
) -> Result<CommentModel, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let comment = CommentEntity::find_by_id(comment_id)
        .one(conn)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(
            "Comment not found".to_string(),
        ))?;

    let mut comment_active: CommentActiveModel = comment.into();
    comment_active.content = Set(content.to_string());
    comment_active.updated_at = Set(Some(chrono::Utc::now()));

    let updated_comment = comment_active.update(conn).await?;
    Ok(updated_comment)
}
