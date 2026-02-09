use mofumofu_entity::hashtags::{
    Column as HashtagColumn, Entity as HashtagEntity, Model as HashtagModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};

pub async fn repository_find_hashtag_by_name<C>(
    conn: &C,
    name: String,
) -> Result<Option<HashtagModel>, Errors>
where
    C: ConnectionTrait,
{
    let hashtag = HashtagEntity::find()
        .filter(HashtagColumn::Name.eq(name))
        .one(conn)
        .await?;

    Ok(hashtag)
}
