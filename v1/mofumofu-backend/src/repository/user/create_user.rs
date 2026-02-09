use crate::dto::user::request::create::CreateUserRequest;
use crate::entity::common::UserRole;
use crate::entity::users::{ActiveModel as UserActiveModel, Model as UserModel};
use crate::service::error::errors::Errors;
use crate::utils::crypto::hash_password;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};
use uuid::Uuid;

pub async fn repository_create_user<C>(
    txn: &C,
    payload: CreateUserRequest,
) -> Result<UserModel, Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let hashed_password = hash_password(&payload.password)?;

    let new_user = UserActiveModel {
        id: Default::default(),
        name: Set(payload.name),
        handle: Set(payload.handle),
        bio: Set(None),
        location: Set(None),
        website: Set(None),
        email: Set(payload.email),
        password: Set(Some(hashed_password)),
        is_verified: Set(false),
        profile_image: Default::default(),
        banner_image: Default::default(),
        follower_count: Set(0),
        following_count: Set(0),
        created_at: Default::default(),
        role: Set(UserRole::Member),
    };

    let user = new_user.insert(txn).await?;

    Ok(user)
}
