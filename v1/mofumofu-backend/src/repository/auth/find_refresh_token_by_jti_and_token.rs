use crate::service::error::errors::Errors;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ConnectionTrait, EntityTrait};
use uuid::Uuid;

pub async fn repository_find_refresh_token_by_jti_and_token<C>(
    conn: &C,
    jti: Uuid,
    refresh_token: String,
) -> Result<Option<crate::entity::user_refresh_tokens::Model>, Errors>
where
    C: ConnectionTrait,
{
    let token = crate::entity::user_refresh_tokens::Entity::find()
        .filter(crate::entity::user_refresh_tokens::Column::Id.eq(jti))
        .filter(crate::entity::user_refresh_tokens::Column::RefreshToken.eq(refresh_token))
        .filter(crate::entity::user_refresh_tokens::Column::RevokedAt.is_null())
        .one(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))?;

    Ok(token)
}
