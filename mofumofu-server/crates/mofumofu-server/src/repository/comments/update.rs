use mofumofu_entity::comments::{
    ActiveModel as CommentActiveModel, Entity as CommentEntity, Model as CommentModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel, Set};
use uuid::Uuid;

#[derive(Default)]
pub struct CommentUpdateParams {
    pub content: Option<String>,
}

pub async fn repository_update_comment<C>(
    conn: &C,
    comment_id: Uuid,
    params: CommentUpdateParams,
) -> Result<CommentModel, Errors>
where
    C: ConnectionTrait,
{
    let comment = CommentEntity::find_by_id(comment_id)
        .one(conn)
        .await?
        .ok_or(Errors::CommentNotFound)?;

    let mut comment_active: CommentActiveModel = comment.into_active_model();

    if let Some(content) = params.content {
        comment_active.content = Set(content);
    }

    let updated_comment = comment_active.update(conn).await?;
    Ok(updated_comment)
}
