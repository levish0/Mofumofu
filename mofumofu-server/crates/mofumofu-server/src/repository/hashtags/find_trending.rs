use mofumofu_entity::hashtags::{
    Column as HashtagColumn, Entity as HashtagEntity, Model as HashtagModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ConnectionTrait, EntityTrait, QueryOrder, QuerySelect};

pub async fn repository_find_trending_hashtags<C>(
    conn: &C,
    limit: u64,
) -> Result<Vec<HashtagModel>, Errors>
where
    C: ConnectionTrait,
{
    let hashtags = HashtagEntity::find()
        .order_by_desc(HashtagColumn::UsageCount)
        .limit(limit)
        .all(conn)
        .await?;

    Ok(hashtags)
}
