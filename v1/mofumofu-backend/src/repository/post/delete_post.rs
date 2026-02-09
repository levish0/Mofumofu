use crate::entity::posts::{Column, Entity as PostEntity};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait};
use uuid::Uuid;

pub async fn repository_delete_post<C>(conn: &C, slug: &str, user_uuid: &Uuid) -> Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let post = PostEntity::find()
        .filter(Column::Slug.eq(slug))
        .filter(Column::UserId.eq(*user_uuid))
        .one(conn)
        .await?
        .ok_or(Errors::PostNotFound)?;

    PostEntity::delete_by_id(post.id).exec(conn).await?;

    Ok(())
}
