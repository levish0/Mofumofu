use serde::Deserialize;
use serde_json::Value as JsonValue;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateDraftRequest {
    pub title: Option<Option<String>>,
    pub slug: Option<Option<String>>,
    pub content: Option<Option<String>>,
    #[schema(value_type = Option<Option<Object>>)]
    pub metadata: Option<Option<JsonValue>>,
}
