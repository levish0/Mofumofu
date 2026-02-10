use mofumofu_dto::user_roles::{GrantRoleRequest, RevokeRoleRequest, UserIdPath, UserRoleResponse};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::grant_role::grant_role,
        super::revoke_role::revoke_role,
        super::get_user_roles::get_user_roles,
    ),
    components(
        schemas(
            GrantRoleRequest,
            RevokeRoleRequest,
            UserRoleResponse,
            UserIdPath,
        )
    ),
    tags(
        (name = "User Roles", description = "User role management endpoints")
    )
)]
pub struct UserRolesApiDoc;
