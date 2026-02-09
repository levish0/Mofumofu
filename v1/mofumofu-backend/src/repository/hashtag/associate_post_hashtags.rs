use crate::entity::common::{ActionType, TargetType};
use crate::entity::hash_tags::{
    ActiveModel as HashTagActiveModel, Column as HashTagColumn, Entity as HashTagEntity,
};
use crate::entity::post_hash_tags::{
    ActiveModel as PostHashTagActiveModel, Entity as PostHashTagEntity,
};
use crate::repository::system_events::log_event::repository_log_event;
use crate::service::error::errors::Errors;
use crate::utils::hashtag_normalizer::normalize_hashtag;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionSession, TransactionTrait};
use uuid::Uuid;

pub async fn repository_associate_post_hashtags<C>(
    conn: &C,
    post_id: Uuid,
    hashtag_names: &[String],
    user_id: Uuid,
) -> Result<Vec<Uuid>, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;
    let mut hashtag_ids = Vec::new();

    for tag_name in hashtag_names {
        let normalized_name = normalize_hashtag(tag_name);

        if normalized_name.is_empty() {
            continue;
        }

        let hashtag = match HashTagEntity::find()
            .filter(HashTagColumn::Name.eq(&normalized_name))
            .one(&txn)
            .await?
        {
            Some(tag) => {
                let mut updated_tag: HashTagActiveModel = tag.clone().into();
                updated_tag.usage_count = Set(tag.usage_count + 1);
                updated_tag.last_used_at = Set(Some(chrono::Utc::now()));
                updated_tag.update(&txn).await?
            }
            None => {
                let new_hashtag = HashTagActiveModel {
                    id: Set(Uuid::new_v4()),
                    name: Set(normalized_name),
                    usage_count: Set(1),
                    created_at: Set(chrono::Utc::now()),
                    last_used_at: Set(Some(chrono::Utc::now())),
                };
                let created_hashtag = new_hashtag.insert(&txn).await?;

                // 새 해시태그 생성 이벤트 로깅
                repository_log_event(
                    conn,
                    Some(user_id),
                    ActionType::HashtagCreated,
                    Some(created_hashtag.id),
                    Some(TargetType::Hashtag),
                    None,
                )
                .await;

                created_hashtag
            }
        };

        let post_hashtag = PostHashTagActiveModel {
            id: Set(Uuid::new_v4()),
            post_id: Set(post_id),
            hash_tag_id: Set(hashtag.id),
            created_at: Set(chrono::Utc::now()),
        };

        post_hashtag.insert(&txn).await?;
        hashtag_ids.push(hashtag.id);
    }

    txn.commit().await?;
    Ok(hashtag_ids)
}
