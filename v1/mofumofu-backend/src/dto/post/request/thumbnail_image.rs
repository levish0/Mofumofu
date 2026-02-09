use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct PostThumbnailForm {
    pub post_id: Uuid,
    #[schema(format = Binary, content_media_type = "image/*")]
    file: String,
}
