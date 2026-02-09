use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn require_verified_user<C>(conn: &C, claims: &AccessTokenClaims) -> ServiceResult<()>
where
    C: ConnectionTrait,
{
    let user = repository_find_user_by_uuid(conn, &claims.sub)
        .await?
        .ok_or(Errors::UserNotFound)?;

    if !user.is_verified {
        return Err(Errors::UserNotVerified);
    }

    Ok(())
}
