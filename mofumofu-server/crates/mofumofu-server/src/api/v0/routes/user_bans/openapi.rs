use mofumofu_dto::user_bans::{BanUserRequest, UnbanUserRequest, UserBanResponse};
use mofumofu_dto::user_roles::UserIdPath;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::ban_user::ban_user,
        super::unban_user::unban_user,
        super::get_user_bans::get_user_bans,
    ),
    components(
        schemas(
            BanUserRequest,
            UnbanUserRequest,
            UserBanResponse,
            UserIdPath,
        )
    ),
    tags(
        (name = "User Bans", description = "User ban management endpoints")
    )
)]
pub struct UserBansApiDoc;
