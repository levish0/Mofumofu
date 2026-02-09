use crate::entity::comments::{ActiveModel as CommentActiveModel, Entity as CommentEntity};
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, Set};
use uuid::Uuid;

pub async fn repository_delete_comment<C>(
    conn: &C,
    comment_id: Uuid,
) -> Result<bool, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let comment = CommentEntity::find_by_id(comment_id).one(conn).await?;

    match comment {
        Some(comment_model) => {
            let mut comment_active: CommentActiveModel = comment_model.into();
            comment_active.is_deleted = Set(true);
            comment_active.updated_at = Set(Some(chrono::Utc::now()));

            comment_active.update(conn).await?;
            Ok(true)
        }
        None => Ok(false),
    }
}
