use mofumofu_entity::users::{Column as UserColumn, Entity as UserEntity};
use mofumofu_errors::errors::Errors;
use sea_orm::sea_query::Expr;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_increment_user_follower_count<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<(), Errors>
where
    C: ConnectionTrait,
{
    UserEntity::update_many()
        .col_expr(
            UserColumn::FollowerCount,
            Expr::col(UserColumn::FollowerCount).add(1),
        )
        .filter(UserColumn::Id.eq(user_id))
        .exec(conn)
        .await?;

    Ok(())
}
