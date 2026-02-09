use crate::entity::common::LikeTargetType;
use crate::entity::likes::{ActiveModel as LikesActiveModel, Model as LikesModel};
use crate::entity::posts::{Column as PostColumn, Entity as PostEntity, Relation as PostRelation};
use crate::entity::users::{Column as UserColumn, Entity as UserEntity};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, JoinType, QueryFilter,
    QuerySelect, RelationTrait, Set,
};
use uuid::Uuid;

pub async fn repository_create_like_by_post_id<C>(
    conn: &C,
    user_id: Uuid,
    post_id: Uuid,
) -> Result<LikesModel, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    repository_create_like(conn, user_id, LikeTargetType::Post, post_id).await
}

pub async fn repository_create_like_by_comment_id<C>(
    conn: &C,
    user_id: Uuid,
    comment_id: Uuid,
) -> Result<LikesModel, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    repository_create_like(conn, user_id, LikeTargetType::Comment, comment_id).await
}

pub async fn repository_create_like<C>(
    conn: &C,
    user_id: Uuid,
    target_type: LikeTargetType,
    target_id: Uuid,
) -> Result<LikesModel, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let (post_id, comment_id) = match target_type {
        LikeTargetType::Post => (Some(target_id), None),
        LikeTargetType::Comment => (None, Some(target_id)),
    };

    let new_like = LikesActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        post_id: Set(post_id),
        comment_id: Set(comment_id),
        target_type: Set(target_type),
        created_at: Default::default(),
    };

    let created_like = new_like.insert(conn).await?;
    Ok(created_like)
}
