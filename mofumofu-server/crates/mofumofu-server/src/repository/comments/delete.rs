use mofumofu_entity::comments::{
    ActiveModel as CommentActiveModel, Entity as CommentEntity, Model as CommentModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel, Set};
use uuid::Uuid;

/// Soft delete: sets deleted_at timestamp instead of removing the row
pub async fn repository_soft_delete_comment<C>(
    conn: &C,
    comment_id: Uuid,
) -> Result<CommentModel, Errors>
where
    C: ConnectionTrait,
{
    let comment = CommentEntity::find_by_id(comment_id)
        .one(conn)
        .await?
        .ok_or(Errors::CommentNotFound)?;

    let mut comment_active: CommentActiveModel = comment.into_active_model();
    comment_active.deleted_at = Set(Some(chrono::Utc::now()));

    let updated_comment = comment_active.update(conn).await?;
    Ok(updated_comment)
}
