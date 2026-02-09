use crate::entity::posts::{
    Column as PostColumn, Entity as PostEntity, Model as PostModel, Relation as PostRelation,
};
use crate::entity::users::{Column as UserColumn, Entity as UserEntity, Relation as UserRelation};
use crate::service::error::errors::Errors;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};

pub async fn repository_find_post_by_handle_and_slug<C>(
    conn: &C,
    handle: &str,
    slug: &str,
) -> Result<Option<PostModel>, Errors>
where
    C: ConnectionTrait,
{
    Ok(PostEntity::find()
        .join(JoinType::InnerJoin, PostRelation::User.def())
        .filter(UserColumn::Handle.eq(handle))
        .filter(PostColumn::Slug.eq(slug))
        .one(conn)
        .await?)
}
