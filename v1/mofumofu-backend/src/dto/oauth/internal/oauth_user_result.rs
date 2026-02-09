use crate::entity::users::Model as UserModel;
#[derive(Debug)]
pub struct OAuthUserResult {
    pub user: UserModel,
    pub is_new_user: bool,
}
