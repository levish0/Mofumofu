use super::super::find_list::PostFilter;
use mofumofu_entity::posts::{Column as PostColumn, Entity as PostEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use uuid::Uuid;

pub async fn repository_exists_older_post<C>(
    conn: &C,
    filter: &PostFilter,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let mut query = PostEntity::find().filter(PostColumn::Id.lt(cursor_id));

    if let Some(user_id) = filter.user_id {
        query = query.filter(PostColumn::UserId.eq(user_id));
    }

    if let Some(true) = filter.published_only {
        query = query.filter(PostColumn::PublishedAt.is_not_null());
    }

    let count = query.limit(1).count(conn).await?;
    Ok(count > 0)
}
