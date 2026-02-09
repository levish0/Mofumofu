use crate::entity::common::OAuthProvider;
use crate::entity::user_oauth_connections::{Column, Entity};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_delete_oauth_connection<C>(
    conn: &C,
    user_id: &Uuid,
    provider: &OAuthProvider,
) -> Result<(), sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    Entity::delete_many()
        .filter(Column::UserId.eq(*user_id))
        .filter(Column::Provider.eq(provider.clone()))
        .exec(conn)
        .await?;

    Ok(())
}
