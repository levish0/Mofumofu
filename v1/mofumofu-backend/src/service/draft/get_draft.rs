use crate::dto::draft::response::draft_info::DraftInfo;
use crate::repository::draft::get_draft_by_id::repository_get_draft_by_id;
use crate::service::error::errors::{Errors, ServiceResult};
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_get_draft<C>(
    conn: &C,
    draft_id: &Uuid,
    user_uuid: &Uuid,
) -> ServiceResult<DraftInfo>
where
    C: ConnectionTrait,
{
    let draft = repository_get_draft_by_id(conn, draft_id, user_uuid).await?;

    let draft = draft.ok_or(Errors::DraftNotFound)?;

    Ok(DraftInfo {
        draft_id: draft.id,
        title: draft.title,
        thumbnail_image: draft.thumbnail_image,
        summary: draft.summary,
        content: draft.content,
        slug: draft.slug,
        created_at: draft.created_at,
        updated_at: draft.updated_at,
    })
}
