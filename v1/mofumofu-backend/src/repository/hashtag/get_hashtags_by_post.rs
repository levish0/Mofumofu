use crate::entity::hash_tags::{Entity as HashTagEntity, Model as HashTagModel};
use crate::entity::post_hash_tags::{Column as PostHashTagColumn, Entity as PostHashTagEntity};
use crate::service::error::errors::Errors;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use uuid::Uuid;

pub async fn repository_get_hashtags_by_post<C>(
    conn: &C,
    post_id: Uuid,
) -> Result<Vec<HashTagModel>, Errors>
where
    C: ConnectionTrait,
{
    let hashtags = HashTagEntity::find()
        .join(
            JoinType::InnerJoin,
            crate::entity::hash_tags::Relation::PostHashTags.def(),
        )
        .filter(PostHashTagColumn::PostId.eq(post_id))
        .all(conn)
        .await?;

    Ok(hashtags)
}

pub async fn repository_get_hashtags_by_posts<C>(
    conn: &C,
    post_ids: &[Uuid],
) -> Result<Vec<(Uuid, Vec<HashTagModel>)>, Errors>
where
    C: ConnectionTrait,
{
    use sea_orm::FromQueryResult;

    #[derive(FromQueryResult)]
    struct PostHashTagResult {
        post_id: Uuid,
        tag_id: Uuid,
        tag_name: String,
        usage_count: i32,
        created_at: chrono::DateTime<chrono::Utc>,
        last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    }

    let results: Vec<PostHashTagResult> = PostHashTagEntity::find()
        .join(
            JoinType::InnerJoin,
            crate::entity::post_hash_tags::Relation::HashTag.def(),
        )
        .filter(PostHashTagColumn::PostId.is_in(post_ids.iter().cloned()))
        .select_only()
        .column(PostHashTagColumn::PostId)
        .column_as(crate::entity::hash_tags::Column::Id, "tag_id")
        .column_as(crate::entity::hash_tags::Column::Name, "tag_name")
        .column_as(crate::entity::hash_tags::Column::UsageCount, "usage_count")
        .column_as(crate::entity::hash_tags::Column::CreatedAt, "created_at")
        .column_as(crate::entity::hash_tags::Column::LastUsedAt, "last_used_at")
        .into_model::<PostHashTagResult>()
        .all(conn)
        .await?;

    let mut post_hashtags: std::collections::HashMap<Uuid, Vec<HashTagModel>> =
        std::collections::HashMap::new();

    for result in results {
        let hashtag = HashTagModel {
            id: result.tag_id,
            name: result.tag_name,
            usage_count: result.usage_count,
            created_at: result.created_at,
            last_used_at: result.last_used_at,
        };

        post_hashtags
            .entry(result.post_id)
            .or_insert_with(Vec::new)
            .push(hashtag);
    }

    Ok(post_hashtags.into_iter().collect())
}
