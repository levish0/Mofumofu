use crate::entity::common::OAuthProvider;
use crate::entity::user_oauth_connections::ActiveModel as OAuthConnectionActiveModel;
use crate::service::error::errors::Errors;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};
use tracing::error;
use uuid::Uuid;

pub async fn repository_create_oauth_connection<C>(
    txn: &C,
    user_id: &Uuid,
    provider: OAuthProvider,
    provider_user_id: &str,
) -> Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let oauth_connection = OAuthConnectionActiveModel {
        id: Default::default(),
        user_id: Set(*user_id),
        provider: Set(provider),
        provider_user_id: Set(provider_user_id.to_string()),
        created_at: Set(Utc::now()),
    };

    oauth_connection.insert(txn).await.map_err(|e| {
        error!("Failed to create OAuth connection: {:?}", e);
        Errors::DatabaseError(e.to_string())
    })?;

    Ok(())
}
