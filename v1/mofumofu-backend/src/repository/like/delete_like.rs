use crate::entity::common::LikeTargetType;
use crate::entity::likes::{
    ActiveModel as LikesActiveModel, Column as LikesColumn, Entity as LikesEntity,
    Relation as LikesRelation,
};
use crate::entity::posts::{Column as PostColumn, Entity as PostEntity, Relation as PostRelation};
use crate::entity::users::{Column as UserColumn, Entity as UserEntity};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, JoinType, QueryFilter,
    QuerySelect, RelationTrait,
};
use uuid::Uuid;

pub async fn repository_delete_like_by_post_id<C>(
    conn: &C,
    user_id: Uuid,
    post_id: Uuid,
) -> Result<bool, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    repository_delete_like(conn, user_id, LikeTargetType::Post, post_id).await
}

pub async fn repository_delete_like_by_comment_id<C>(
    conn: &C,
    user_id: Uuid,
    comment_id: Uuid,
) -> Result<bool, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    repository_delete_like(conn, user_id, LikeTargetType::Comment, comment_id).await
}

pub async fn repository_delete_like<C>(
    conn: &C,
    user_id: Uuid,
    target_type: LikeTargetType,
    target_id: Uuid,
) -> Result<bool, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let mut query = LikesEntity::find()
        .filter(LikesColumn::UserId.eq(user_id))
        .filter(LikesColumn::TargetType.eq(target_type.clone()));

    query = match target_type {
        LikeTargetType::Post => query.filter(LikesColumn::PostId.eq(target_id)),
        LikeTargetType::Comment => query.filter(LikesColumn::CommentId.eq(target_id)),
    };

    let existing_like = query.one(conn).await?;

    match existing_like {
        Some(like_record) => {
            let like_active_model: LikesActiveModel = like_record.into();
            like_active_model.delete(conn).await?;
            Ok(true)
        }
        None => Ok(false),
    }
}
