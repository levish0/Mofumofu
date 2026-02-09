use crate::dto::comment::request::get_comments::CommentSortOrder;
use crate::entity::comments::{
    Column as CommentColumn, Entity as CommentEntity, Model as CommentModel,
};
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

pub async fn repository_get_comments<C>(
    conn: &C,
    post_id: Uuid,
    page: u32,
    per_page: u32,
    sort: CommentSortOrder,
) -> Result<Vec<CommentModel>, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let offset = (page - 1) * per_page;

    let mut query = CommentEntity::find()
        .filter(CommentColumn::PostId.eq(post_id))
        .filter(CommentColumn::ParentId.is_null()); // 부모 댓글만

    // 정렬 적용
    query = match sort {
        CommentSortOrder::Latest => query.order_by_desc(CommentColumn::CreatedAt),
        CommentSortOrder::Oldest => query.order_by_asc(CommentColumn::CreatedAt),
        CommentSortOrder::Popular => query.order_by_desc(CommentColumn::LikeCount),
    };

    let comments = query
        .offset(offset as u64)
        .limit(per_page as u64)
        .all(conn)
        .await?;

    Ok(comments)
}

pub async fn repository_get_replies<C>(
    conn: &C,
    parent_comment_id: Uuid,
    page: u32,
    per_page: u32,
    sort: CommentSortOrder,
) -> Result<Vec<CommentModel>, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let offset = (page - 1) * per_page;

    let mut query = CommentEntity::find().filter(CommentColumn::ParentId.eq(parent_comment_id));

    // 정렬 적용
    query = match sort {
        CommentSortOrder::Latest => query.order_by_desc(CommentColumn::CreatedAt),
        CommentSortOrder::Oldest => query.order_by_asc(CommentColumn::CreatedAt),
        CommentSortOrder::Popular => query.order_by_desc(CommentColumn::LikeCount),
    };

    let replies = query
        .offset(offset as u64)
        .limit(per_page as u64)
        .all(conn)
        .await?;

    Ok(replies)
}

pub async fn repository_count_comments<C>(conn: &C, post_id: Uuid) -> Result<u64, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let count = CommentEntity::find()
        .filter(CommentColumn::PostId.eq(post_id))
        .filter(CommentColumn::ParentId.is_null()) // 부모 댓글만
        .count(conn)
        .await?;

    Ok(count)
}

pub async fn repository_count_replies<C>(
    conn: &C,
    parent_comment_id: Uuid,
) -> Result<u64, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let count = CommentEntity::find()
        .filter(CommentColumn::ParentId.eq(parent_comment_id))
        .count(conn)
        .await?;

    Ok(count)
}
