use crate::dto::user::internal::update_user::UpdateUserFields;
use crate::entity::users::{
    ActiveModel as UserActiveModel, Entity as UserEntity, Model as UserModel,
};
use crate::service::error::errors::Errors;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ConnectionTrait, EntityTrait, NotSet, Set, TransactionTrait,
};
use uuid::Uuid;

pub async fn repository_update_user<C>(
    txn: &C,
    user_uuid: &Uuid,
    fields: UpdateUserFields,
) -> Result<UserModel, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    // 사용자 존재 여부 확인
    let existing_user = UserEntity::find_by_id(*user_uuid)
        .one(txn)
        .await?
        .ok_or(Errors::UserNotFound)?;

    // 업데이트할 필드만 설정
    let user_active_model = UserActiveModel {
        id: Set(existing_user.id),
        name: fields.name.map_or(NotSet, Set),
        handle: fields.handle.map_or(NotSet, Set),
        bio: fields.bio.map_or(NotSet, Set),
        location: fields.location.map_or(NotSet, Set),
        website: fields.website.map_or(NotSet, Set),
        email: fields.email.map_or(NotSet, Set),
        password: fields.password.map_or(NotSet, Set),
        is_verified: fields.is_verified.map_or(NotSet, Set),
        profile_image: fields.profile_image.map_or(NotSet, Set),
        banner_image: fields.banner_image.map_or(NotSet, Set),
        follower_count: NotSet,
        following_count: NotSet,
        created_at: NotSet,
        role: NotSet,
    };

    // 업데이트 실행
    let updated_user = user_active_model.update(txn).await?;

    Ok(updated_user)
}
