use crate::dto::user::request::update_profile::UpdateProfileRequest;
use crate::dto::user::response::info::UserInfoResponse;

use crate::dto::user::internal::update_user::UpdateUserFields;
use crate::repository::user::find_user_by_uuid::repository_find_user_by_uuid;
use crate::repository::user::update_user::repository_update_user;
use crate::service::error::errors::{Errors, ServiceResult};
use crate::utils::crypto::{hash_password, verify_password};
use sea_orm::{ConnectionTrait, TransactionSession, TransactionTrait};
use uuid::Uuid;

pub async fn service_update_user_profile<C>(
    conn: &C,
    user_uuid: &Uuid,
    payload: UpdateProfileRequest,
) -> ServiceResult<UserInfoResponse>
where
    C: ConnectionTrait + TransactionTrait,
{
    let txn = conn.begin().await?;

    // 사용자 정보 조회
    let user = repository_find_user_by_uuid(&txn, user_uuid)
        .await?
        .ok_or(Errors::UserNotFound)?;

    // 패스워드 업데이트가 요청된 경우 현재 패스워드 확인
    let new_password_hash = if let Some(new_password) = &payload.password {
        // 새로운 패스워드가 제공된 경우 현재 패스워드 확인 필요
        let current_password = payload
            .current_password
            .as_ref()
            .ok_or(Errors::PasswordRequiredForUpdate)?;

        // OAuth 전용 계정인 경우 (패스워드가 없는 경우) 에러 반환
        let existing_password_hash = user
            .password
            .as_ref()
            .ok_or(Errors::PasswordCannotUpdateOauthOnly)?;

        // 현재 패스워드 검증
        if let Err(_) = verify_password(current_password, existing_password_hash) {
            return Err(Errors::PasswordIncorrect);
        }

        // 새로운 패스워드 해시화
        Some(hash_password(new_password)?)
    } else {
        // 패스워드 업데이트가 없는 경우
        if payload.current_password.is_some() {
            return Err(Errors::PasswordNewPasswordMissing);
        }
        None
    };

    // payload에서 업데이트할 필드만 추출
    let update_fields = UpdateUserFields {
        name: payload.name,
        handle: payload.handle,
        bio: Some(payload.bio),
        location: Some(payload.location),
        website: Some(payload.website),
        password: Some(new_password_hash),
        ..Default::default() // 나머지 필드는 None
    };

    let updated_user = repository_update_user(&txn, user_uuid, update_fields).await?;

    txn.commit().await?;

    Ok(UserInfoResponse {
        name: updated_user.name,
        handle: updated_user.handle,
        email: updated_user.email,
        bio: updated_user.bio,
        location: updated_user.location,
        website: updated_user.website,
        profile_image: updated_user.profile_image,
        banner_image: updated_user.banner_image,
        is_verified: updated_user.is_verified,
        created_at: updated_user.created_at,
    })
}
