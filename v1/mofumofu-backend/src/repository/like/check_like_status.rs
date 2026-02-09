use crate::entity::likes::{
    Column as LikesColumn, Entity as LikesEntity, Relation as LikesRelation,
};
use crate::entity::posts::{Column as PostColumn, Entity as PostEntity, Relation as PostRelation};
use crate::entity::users::{Column as UserColumn, Entity as UserEntity, Relation as UserRelation};
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use uuid::Uuid;

pub async fn repository_check_like_status<C>(
    conn: &C,
    user_id: &Uuid,
    handle: &str,
    slug: &str,
) -> Result<bool, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let like_exists = LikesEntity::find()
        .join(JoinType::InnerJoin, LikesRelation::Post.def())
        .join(JoinType::InnerJoin, PostRelation::User.def())
        .filter(LikesColumn::UserId.eq(*user_id))
        .filter(PostColumn::Slug.eq(slug))
        .filter(UserColumn::Handle.eq(handle))
        .one(conn)
        .await?;

    Ok(like_exists.is_some())
}

pub async fn repository_check_like_status_by_post_id<C>(
    conn: &C,
    user_id: &Uuid,
    post_id: &Uuid,
) -> Result<bool, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let like_exists = LikesEntity::find()
        .filter(LikesColumn::UserId.eq(*user_id))
        .filter(LikesColumn::PostId.eq(*post_id))
        .one(conn)
        .await?;

    Ok(like_exists.is_some())
}

pub async fn repository_check_like_status_by_comment_id<C>(
    conn: &C,
    user_id: &Uuid,
    comment_id: &Uuid,
) -> Result<bool, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let like_exists = LikesEntity::find()
        .filter(LikesColumn::UserId.eq(*user_id))
        .filter(LikesColumn::CommentId.eq(*comment_id))
        .one(conn)
        .await?;

    Ok(like_exists.is_some())
}
