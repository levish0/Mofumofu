use mofumofu_entity::comments::{Column as CommentColumn, Entity as CommentEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_decrement_comment_like_count<C>(
    conn: &C,
    comment_id: Uuid,
) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    CommentEntity::update_many()
        .col_expr(
            CommentColumn::LikeCount,
            Expr::col(CommentColumn::LikeCount).sub(1),
        )
        .filter(CommentColumn::Id.eq(comment_id))
        .exec(conn)
        .await?;

    Ok(())
}
