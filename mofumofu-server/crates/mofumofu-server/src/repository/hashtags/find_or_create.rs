use mofumofu_entity::hashtags::{
    ActiveModel as HashtagActiveModel, Column as HashtagColumn, Entity as HashtagEntity,
    Model as HashtagModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};

pub async fn repository_find_or_create_hashtag<C>(
    conn: &C,
    name: String,
) -> Result<HashtagModel, Errors>
where
    C: ConnectionTrait,
{
    let existing = HashtagEntity::find()
        .filter(HashtagColumn::Name.eq(&name))
        .one(conn)
        .await?;

    if let Some(hashtag) = existing {
        return Ok(hashtag);
    }

    let new_hashtag = HashtagActiveModel {
        id: Default::default(),
        name: Set(name),
        usage_count: Set(0),
        last_used_at: Set(None),
        created_at: Default::default(),
    };

    let hashtag = new_hashtag.insert(conn).await?;
    Ok(hashtag)
}
