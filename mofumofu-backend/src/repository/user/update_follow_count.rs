use crate::entity::users::{Column as UsersColumn, Entity as UsersEntity};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ExprTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_increment_user_follower_count<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    UsersEntity::update_many()
        .filter(UsersColumn::Id.eq(user_id))
        .col_expr(
            UsersColumn::FollowerCount,
            UsersColumn::FollowerCount.into_expr().add(1),
        )
        .exec(conn)
        .await?;

    Ok(())
}

pub async fn repository_decrement_user_follower_count<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    UsersEntity::update_many()
        .filter(UsersColumn::Id.eq(user_id))
        .col_expr(
            UsersColumn::FollowerCount,
            UsersColumn::FollowerCount.into_expr().sub(1),
        )
        .exec(conn)
        .await?;

    Ok(())
}

pub async fn repository_increment_user_following_count<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    UsersEntity::update_many()
        .filter(UsersColumn::Id.eq(user_id))
        .col_expr(
            UsersColumn::FollowingCount,
            UsersColumn::FollowingCount.into_expr().add(1),
        )
        .exec(conn)
        .await?;

    Ok(())
}

pub async fn repository_decrement_user_following_count<C>(
    conn: &C,
    user_id: Uuid,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    UsersEntity::update_many()
        .filter(UsersColumn::Id.eq(user_id))
        .col_expr(
            UsersColumn::FollowingCount,
            UsersColumn::FollowingCount.into_expr().sub(1),
        )
        .exec(conn)
        .await?;

    Ok(())
}
