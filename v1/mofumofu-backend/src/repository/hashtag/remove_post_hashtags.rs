use crate::entity::post_hash_tags::{Column, Entity as PostHashTagEntity};
use crate::service::error::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, TransactionTrait};
use uuid::Uuid;

pub async fn repository_remove_post_hashtags<C>(conn: &C, post_id: Uuid) -> Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    PostHashTagEntity::delete_many()
        .filter(Column::PostId.eq(post_id))
        .exec(conn)
        .await?;

    Ok(())
}
