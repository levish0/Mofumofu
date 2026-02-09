use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct ProfileAvatarForm {
    #[schema(format = Binary, content_media_type = "image/*")]
    file: String,
}
