use crate::validator::string_validator::validate_not_blank;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetPostBySlugRequest {
    #[validate(length(
        min = 1,
        max = 20,
        message = "Handle must be between 1 and 20 characters."
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub handle: String,

    #[validate(length(
        min = 1,
        max = 200,
        message = "Slug must be between 1 and 200 characters."
    ))]
    #[validate(custom(function = "validate_not_blank"))]
    pub slug: String,
}
