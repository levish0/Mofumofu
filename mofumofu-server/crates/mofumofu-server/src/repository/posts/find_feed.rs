use chrono::{DateTime, Utc};
use mofumofu_dto::posts::PostSortOrder;
use mofumofu_entity::posts::{Column as PostColumn, Entity as PostEntity, Model as PostModel};
use mofumofu_errors::errors::Errors;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};

pub struct PostFeedFilter {
    pub published_at_after: Option<DateTime<Utc>>,
}

fn apply_feed_filters(
    mut query: sea_orm::Select<PostEntity>,
    filter: &PostFeedFilter,
) -> sea_orm::Select<PostEntity> {
    query = query.filter(PostColumn::PublishedAt.is_not_null());

    if let Some(after) = filter.published_at_after {
        query = query.filter(PostColumn::PublishedAt.gte(after));
    }

    query
}

pub async fn repository_find_post_feed<C>(
    conn: &C,
    filter: &PostFeedFilter,
    sort: &PostSortOrder,
    page: u32,
    page_size: u32,
) -> Result<Vec<PostModel>, Errors>
where
    C: ConnectionTrait,
{
    let offset = ((page - 1) * page_size) as u64;

    let mut query = PostEntity::find();
    query = apply_feed_filters(query, filter);

    query = match sort {
        PostSortOrder::Latest => query.order_by_desc(PostColumn::CreatedAt),
        PostSortOrder::Popular => query
            .order_by_desc(PostColumn::LikeCount)
            .order_by_desc(PostColumn::CreatedAt),
        PostSortOrder::Oldest => query.order_by_asc(PostColumn::CreatedAt),
    };

    let posts = query
        .offset(offset)
        .limit(page_size as u64)
        .all(conn)
        .await?;

    Ok(posts)
}

pub async fn repository_count_post_feed<C>(
    conn: &C,
    filter: &PostFeedFilter,
) -> Result<u64, Errors>
where
    C: ConnectionTrait,
{
    let query = PostEntity::find();
    let query = apply_feed_filters(query, filter);
    let count = query.count(conn).await?;
    Ok(count)
}
