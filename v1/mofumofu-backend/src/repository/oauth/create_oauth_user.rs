use crate::dto::user::request::create::CreateUserRequest;
use crate::entity::common::UserRole;
use crate::entity::users::ActiveModel as UserActiveModel;
use crate::service::error::errors::Errors;
use crate::utils::crypto::hash_password;
use sea_orm::{ActiveModelTrait, ConnectionTrait, Set, TransactionTrait};

pub async fn repository_create_oauth_user<C>(
    txn: &C,
    email: &str,
    name: &str,
    handle: &str,
    profile_image: Option<String>,
) -> Result<(), Errors>
where
    C: ConnectionTrait + TransactionTrait,
{
    let new_user = UserActiveModel {
        id: Default::default(),
        name: Set(name.to_string()),
        handle: Set(handle.to_string()),
        bio: Set(None),
        location: Set(None),
        website: Set(None),
        email: Set(email.to_string()),
        password: Set(None),
        is_verified: Set(true),
        profile_image: Set(profile_image),
        banner_image: Set(None),
        follower_count: Set(0),
        following_count: Set(0),
        created_at: Default::default(),
        role: Set(UserRole::Member),
    };

    new_user.insert(txn).await?;

    Ok(())
}
