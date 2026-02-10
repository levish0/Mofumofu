use mofumofu_entity::posts::{Column as PostColumn, Entity as PostEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_decrement_post_like_count<C>(conn: &C, post_id: Uuid) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    PostEntity::update_many()
        .col_expr(
            PostColumn::LikeCount,
            Expr::col(PostColumn::LikeCount).sub(1),
        )
        .filter(PostColumn::Id.eq(post_id))
        .exec(conn)
        .await?;

    Ok(())
}
