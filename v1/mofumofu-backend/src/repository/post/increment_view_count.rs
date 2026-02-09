use crate::entity::posts::{self, Entity as Posts};
use crate::service::error::errors::Errors;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn repository_increment_view_count<C>(conn: &C, post_id: &Uuid) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    let post = Posts::find()
        .filter(posts::Column::Id.eq(*post_id))
        .one(conn)
        .await?
        .ok_or(Errors::PostNotFound)?;

    let mut active_post: posts::ActiveModel = post.into();
    active_post.view_count = Set(active_post.view_count.unwrap() + 1);

    active_post.update(conn).await?;

    Ok(())
}
