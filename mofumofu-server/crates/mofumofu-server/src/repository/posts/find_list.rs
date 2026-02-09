use mofumofu_dto::pagination::CursorDirection;
use mofumofu_entity::posts::{Column as PostColumn, Entity as PostEntity, Model as PostModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct PostFilter {
    pub user_id: Option<Uuid>,
    pub published_only: Option<bool>,
}

pub async fn repository_find_posts<C>(
    conn: &C,
    filter: &PostFilter,
    cursor_id: Option<Uuid>,
    cursor_direction: Option<CursorDirection>,
    limit: u64,
) -> Result<Vec<PostModel>, Errors>
where
    C: ConnectionTrait,
{
    let mut query = PostEntity::find();

    if let Some(user_id) = filter.user_id {
        query = query.filter(PostColumn::UserId.eq(user_id));
    }

    if let Some(true) = filter.published_only {
        query = query.filter(PostColumn::PublishedAt.is_not_null());
    }

    if let Some(id) = cursor_id {
        let direction = cursor_direction.unwrap_or(CursorDirection::Older);
        query = match direction {
            CursorDirection::Older => query
                .filter(PostColumn::Id.lt(id))
                .order_by_desc(PostColumn::Id),
            CursorDirection::Newer => query
                .filter(PostColumn::Id.gt(id))
                .order_by_asc(PostColumn::Id),
        };
    } else {
        query = query.order_by_desc(PostColumn::Id);
    }

    let posts = query.limit(limit).all(conn).await?;

    Ok(posts)
}
