use crate::entity::common::OAuthProvider;
use crate::entity::user_oauth_connections::{Column, Entity, Model};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

pub async fn repository_get_user_oauth_connections<C>(
    conn: &C,
    user_id: &Uuid,
) -> Result<Vec<Model>, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    Entity::find()
        .filter(Column::UserId.eq(*user_id))
        .all(conn)
        .await
}

pub async fn repository_get_oauth_providers_by_user_id<C>(
    conn: &C,
    user_id: &Uuid,
) -> Result<Vec<OAuthProvider>, sea_orm::DbErr>
where
    C: ConnectionTrait,
{
    let connections = repository_get_user_oauth_connections(conn, user_id).await?;
    Ok(connections.into_iter().map(|conn| conn.provider).collect())
}
