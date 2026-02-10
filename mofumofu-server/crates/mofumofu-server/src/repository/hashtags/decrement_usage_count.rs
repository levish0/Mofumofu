use mofumofu_entity::hashtags::{Column as HashtagColumn, Entity as HashtagEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_decrement_hashtag_usage_count<C>(
    conn: &C,
    hashtag_id: Uuid,
) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    HashtagEntity::update_many()
        .col_expr(
            HashtagColumn::UsageCount,
            Expr::col(HashtagColumn::UsageCount).sub(1),
        )
        .filter(HashtagColumn::Id.eq(hashtag_id))
        .exec(conn)
        .await?;

    Ok(())
}
