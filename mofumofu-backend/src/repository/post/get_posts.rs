use crate::dto::post::request::PostSortOrder;
use crate::entity::common::{ActionType, TargetType};
use crate::entity::posts::{Column, Entity as PostEntity, Model as PostModel};
use crate::service::error::errors::Errors;
use sea_orm::prelude::Expr;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

pub async fn repository_get_posts<C>(
    conn: &C,
    page: u32,
    page_size: u32,
    sort_order: &PostSortOrder,
) -> Result<Vec<PostModel>, Errors>
where
    C: ConnectionTrait,
{
    let offset = (page - 1) * page_size;

    let mut query = PostEntity::find();

    // 정렬 조건 적용
    // SQL 문자열을 미리 생성하여 lifetime 보장
    let two_weeks_ago = chrono::Utc::now() - chrono::Duration::weeks(2);
    let sql = format!(
        "(SELECT COUNT(*) FROM system_events WHERE target_id = posts.id AND action_type = 'post_viewed' AND target_type = 'post' AND created_at >= '{}')",
        two_weeks_ago.format("%Y-%m-%d %H:%M:%S%.3f+00:00")
    );

    match sort_order {
        PostSortOrder::Latest => {
            query = query.order_by_desc(Column::CreatedAt);
        }
        PostSortOrder::Popular => {
            // 최근 2주간 PostViewed 이벤트 수 기반으로 trending 정렬
            query = query
                .order_by_desc(Expr::cust(&sql))
                .order_by_desc(Column::CreatedAt); // 같은 view count일 때는 최신순
        }
        PostSortOrder::Oldest => {
            query = query.order_by_asc(Column::CreatedAt);
        }
    }

    let posts = query
        .offset(offset as u64)
        .limit(page_size as u64)
        .all(conn)
        .await?;

    Ok(posts)
}

pub async fn repository_get_posts_around_page<C>(
    conn: &C,
    target_page: u32,
    page_size: u32,
    pages_around: u32,
    sort_order: &PostSortOrder,
) -> Result<Vec<PostModel>, Errors>
where
    C: ConnectionTrait,
{
    let start_page = if target_page > pages_around {
        target_page - pages_around
    } else {
        1
    };
    let end_page = target_page + pages_around;

    let start_offset = (start_page - 1) * page_size;
    let total_items = (end_page - start_page + 1) * page_size;

    let mut query = PostEntity::find();

    match sort_order {
        PostSortOrder::Latest => {
            query = query.order_by_desc(Column::CreatedAt);
        }
        PostSortOrder::Popular => {
            query = query.order_by_desc(Column::LikeCount);
        }
        PostSortOrder::Oldest => {
            query = query.order_by_asc(Column::CreatedAt);
        }
    }

    let posts = query
        .offset(start_offset as u64)
        .limit(total_items as u64)
        .all(conn)
        .await?;

    Ok(posts)
}

pub async fn repository_get_posts_count<C>(conn: &C) -> Result<u64, Errors>
where
    C: ConnectionTrait,
{
    let count = PostEntity::find().count(conn).await?;

    Ok(count)
}

pub async fn repository_get_posts_by_ids<C>(
    conn: &C,
    post_ids: &[String],
) -> Result<Vec<PostModel>, Errors>
where
    C: ConnectionTrait,
{
    if post_ids.is_empty() {
        return Ok(Vec::new());
    }

    // String ID들을 Uuid로 변환
    let uuid_ids: Result<Vec<Uuid>, _> = post_ids.iter().map(|id| Uuid::parse_str(id)).collect();

    let uuid_ids =
        uuid_ids.map_err(|e| Errors::BadRequestError(format!("Invalid UUID format: {}", e)))?;

    let posts = PostEntity::find()
        .filter(Column::Id.is_in(uuid_ids))
        .all(conn)
        .await?;

    Ok(posts)
}
