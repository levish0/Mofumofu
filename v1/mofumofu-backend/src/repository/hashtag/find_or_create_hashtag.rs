use crate::entity::hash_tags::{
    ActiveModel as HashTagActiveModel, Column, Entity as HashTagEntity, Model as HashTagModel,
};
use crate::service::error::errors::Errors;
use crate::utils::hashtag_normalizer::normalize_hashtag;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub async fn repository_find_or_create_hashtag<C>(
    conn: &C,
    tag_name: &str,
) -> Result<HashTagModel, Errors>
where
    C: ConnectionTrait,
{
    let normalized_name = normalize_hashtag(tag_name);

    if let Some(existing_tag) = HashTagEntity::find()
        .filter(Column::Name.eq(&normalized_name))
        .one(conn)
        .await?
    {
        return Ok(existing_tag);
    }

    let new_hashtag = HashTagActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(normalized_name),
        usage_count: Set(1),
        created_at: Set(chrono::Utc::now()),
        last_used_at: Set(Some(chrono::Utc::now())),
    };

    let created_tag = new_hashtag.insert(conn).await?;
    Ok(created_tag)
}
