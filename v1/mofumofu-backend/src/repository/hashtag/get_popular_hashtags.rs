use crate::entity::common::{ActionType, TargetType};
use crate::entity::hash_tags::{Column, Entity as HashTagEntity, Model as HashTagModel};
use crate::entity::system_events::{Column as SystemEventColumn, Entity as SystemEventEntity};
use crate::service::error::errors::Errors;
use chrono::{Duration, Utc};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbBackend, EntityTrait, FromQueryResult, QueryFilter, QueryOrder,
    QuerySelect, Statement,
};

pub async fn repository_get_popular_hashtags<C>(
    conn: &C,
    limit: u64,
) -> Result<Vec<HashTagModel>, Errors>
where
    C: ConnectionTrait,
{
    let hashtags = HashTagEntity::find()
        .order_by_desc(Column::UsageCount)
        .limit(limit)
        .all(conn)
        .await?;

    Ok(hashtags)
}

pub async fn repository_get_recent_hashtags<C>(
    conn: &C,
    limit: u64,
) -> Result<Vec<HashTagModel>, Errors>
where
    C: ConnectionTrait,
{
    let hashtags = HashTagEntity::find()
        .order_by_desc(Column::LastUsedAt)
        .limit(limit)
        .all(conn)
        .await?;

    Ok(hashtags)
}

pub async fn repository_get_trending_hashtags<C>(
    conn: &C,
    days: i64,
    limit: u64,
) -> Result<Vec<HashTagModel>, Errors>
where
    C: ConnectionTrait,
{
    let since = Utc::now() - Duration::days(days);

    // 최근 N일간 hashtag_used 액션이 있는 해시태그들을
    // 사용 빈도순으로 정렬해서 가져오기
    let hashtag_ids: Vec<uuid::Uuid> = SystemEventEntity::find()
        .filter(SystemEventColumn::ActionType.eq(ActionType::HashtagUsed))
        .filter(SystemEventColumn::TargetType.eq(TargetType::Hashtag))
        .filter(SystemEventColumn::CreatedAt.gte(since))
        .group_by(SystemEventColumn::TargetId)
        .order_by_desc(SystemEventColumn::TargetId.count())
        .limit(limit)
        .select_only()
        .column(SystemEventColumn::TargetId)
        .into_tuple::<uuid::Uuid>()
        .all(conn)
        .await?;

    // 해당 해시태그들의 정보를 가져오기
    let hashtags = HashTagEntity::find()
        .filter(Column::Id.is_in(hashtag_ids))
        .order_by_desc(Column::UsageCount)
        .all(conn)
        .await?;

    Ok(hashtags)
}
