use crate::entity::posts::{Column as PostColumn, Entity as PostEntity, Model as PostModel};
use crate::entity::users::{Column as UserColumn, Entity as UserEntity, Relation as UserRelation};
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, JoinType, QueryFilter, QueryOrder, QuerySelect,
    RelationTrait,
};
use uuid::Uuid;

pub async fn repository_get_user_posts<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<Vec<PostModel>, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let posts = PostEntity::find()
        .filter(PostColumn::UserId.eq(user_id))
        .order_by_desc(PostColumn::CreatedAt)
        .all(conn)
        .await?;

    Ok(posts)
}
