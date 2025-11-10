use crate::entity::posts::{Column as PostsColumn, Entity as PostsEntity};
use sea_orm::sea_query::Func;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter, Set};
use sea_orm::prelude::Expr::FunctionCall;
use uuid::Uuid;

pub async fn repository_increment_post_like_count<C>(
    conn: &C,
    post_id: Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    PostsEntity::update_many()
        .filter(PostsColumn::Id.eq(post_id))
        .col_expr(
            PostsColumn::LikeCount,
            PostsColumn::LikeCount.into_expr().add(1),
        )
        .exec(conn)
        .await?;

    Ok(())
}

pub async fn repository_decrement_post_like_count<C>(
    conn: &C,
    post_id: Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    PostsEntity::update_many()
        .filter(PostsColumn::Id.eq(post_id))
        .col_expr(
            PostsColumn::LikeCount,
            FunctionCall(Func::greatest([
                PostsColumn::LikeCount.into_expr().sub(1),
                0.into(),
            ])),
        )
        .exec(conn)
        .await?;

    Ok(())
}
