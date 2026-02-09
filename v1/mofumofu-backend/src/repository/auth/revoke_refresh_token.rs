use crate::service::error::errors::Errors;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};

pub async fn repository_revoke_refresh_token<C>(
    conn: &C,
    refresh_token_model: crate::entity::user_refresh_tokens::Model,
    ip_address: Option<String>,
    user_agent: Option<String>,
    revoked_at: chrono::DateTime<chrono::Utc>,
) -> Result<crate::entity::user_refresh_tokens::Model, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let mut revoke_model: crate::entity::user_refresh_tokens::ActiveModel =
        refresh_token_model.into();
    revoke_model.revoked_at = Set(Some(revoked_at));
    revoke_model.ip_address = Set(ip_address);
    revoke_model.user_agent = Set(user_agent);

    revoke_model
        .update(conn)
        .await
        .map_err(|e| Errors::DatabaseError(e.to_string()))
}
