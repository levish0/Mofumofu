use crate::entity::comments::{Column as CommentColumn, Entity as CommentEntity};
use sea_orm::sea_query::Func;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use sea_orm::prelude::Expr::FunctionCall;
use uuid::Uuid;

pub async fn repository_increment_comment_like_count<C>(
    conn: &C,
    comment_id: &Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    CommentEntity::update_many()
        .filter(CommentColumn::Id.eq(*comment_id))
        .col_expr(
            CommentColumn::LikeCount,
            CommentColumn::LikeCount.into_expr().add(1),
        )
        .exec(conn)
        .await?;

    Ok(())
}

pub async fn repository_decrement_comment_like_count<C>(
    conn: &C,
    comment_id: &Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    CommentEntity::update_many()
        .filter(CommentColumn::Id.eq(*comment_id))
        .col_expr(
            CommentColumn::LikeCount,
            FunctionCall(Func::greatest([
                CommentColumn::LikeCount.into_expr().sub(1),
                0.into(),
            ])),
        )
        .exec(conn)
        .await?;

    Ok(())
}
