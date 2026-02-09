use crate::service::error::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait};

pub async fn repository_create_refresh_token<C>(
    conn: &C,
    refresh_model: crate::entity::user_refresh_tokens::ActiveModel,
) -> Result<crate::entity::user_refresh_tokens::Model, Errors>
where
    C: ConnectionTrait,
{
    refresh_model
        .insert(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
}
