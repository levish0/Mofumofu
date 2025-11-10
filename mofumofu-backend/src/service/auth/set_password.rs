use crate::dto::auth::request::set_password::SetPasswordRequest;
use crate::dto::user::internal::update_user::UpdateUserFields;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::repository::user::update_user::repository_update_user;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::utils::crypto::hash_password;
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use tracing::info;
use uuid::Uuid;

pub async fn service_set_password<C>(
    conn: &C,
    user_id: Uuid,
    payload: SetPasswordRequest,
) -> ServiceResult<()>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // 사용자 확인
    let user = repository_find_user_by_uuid(&txn, &user_id)
        .await?
        .ok_or(Errors::UserNotFound)?;

    // 이미 비밀번호가 설정되어 있는지 확인
    if user.password.is_some() {
        return Err(Errors::PasswordAlreadySet);
    }

    // 새 비밀번호 해싱
    let hashed_password = hash_password(&payload.password)?;

    // 사용자 비밀번호 설정
    let update_fields = UpdateUserFields {
        name: None,
        handle: None,
        bio: None,
        location: None,
        website: None,
        email: None,
        password: Some(Some(hashed_password)),
        is_verified: None,
        profile_image: None,
        banner_image: None,
    };

    repository_update_user(&txn, &user_id, update_fields).await?;

    txn.commit().await?;

    info!("Password set successfully for user: {}", user_id);

    Ok(())
}
