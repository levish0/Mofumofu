use crate::entity::posts::{Column as PostColumn, Entity as PostEntity};
use sea_orm::sea_query::Func;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use sea_orm::prelude::Expr::FunctionCall;
use uuid::Uuid;

pub async fn repository_increment_comment_count<C>(
    conn: &C,
    post_id: &Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    PostEntity::update_many()
        .filter(PostColumn::Id.eq(*post_id))
        .col_expr(
            PostColumn::CommentCount,
            PostColumn::CommentCount.into_expr().add(1),
        )
        .exec(conn)
        .await?;

    Ok(())
}

pub async fn repository_decrement_comment_count<C>(
    conn: &C,
    post_id: &Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    PostEntity::update_many()
        .filter(PostColumn::Id.eq(*post_id))
        .col_expr(
            PostColumn::CommentCount,
            FunctionCall(Func::greatest([
                PostColumn::CommentCount.into_expr().sub(1),
                0.into(),
            ])),
        )
        .exec(conn)
        .await?;

    Ok(())
}
