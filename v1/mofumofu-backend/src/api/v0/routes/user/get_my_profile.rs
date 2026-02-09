use crate::dto::auth::internal::access_token::AccessTokenClaims;
use crate::dto::user::response::info::UserInfoResponse;
use crate::service::error::errors::Errors;
use crate::service::user::service_get_user_by_uuid;
use crate::state::AppState;
use axum::Extension;
use axum::extract::State;

// 보호된 API - 사용자 프로필 조회
#[utoipa::path(
    get,
    path = "/v0/user/my_profile",
    responses(
        (status = StatusCode::OK, description = "Successfully retrieved user profile", body = UserInfoResponse),
        (status = StatusCode::UNAUTHORIZED, description = "Unauthorized"),
        (status = StatusCode::NOT_FOUND, description = "User not found"),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Internal Server Error")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "User"
)]
pub async fn get_my_profile(
    state: State<AppState>,
    Extension(claims): Extension<AccessTokenClaims>,
) -> Result<UserInfoResponse, Errors> {
    let user_uuid = claims.sub.clone();

    let user = service_get_user_by_uuid(&state.conn, &user_uuid).await?;

    Ok(user)
}
