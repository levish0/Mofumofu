use crate::dto::draft::response::draft_info::DraftInfo;
use crate::dto::draft::response::get_drafts::GetDraftsResponse;
use crate::repository::draft::get_drafts_by_user::repository_get_drafts_by_user;
use crate::service::error::errors::ServiceResult;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

pub async fn service_get_drafts<C>(conn: &C, user_uuid: &Uuid) -> ServiceResult<GetDraftsResponse>
where
    C: ConnectionTrait,
{
    let drafts = repository_get_drafts_by_user(conn, user_uuid).await?;

    let draft_infos: Vec<DraftInfo> = drafts
        .into_iter()
        .map(|draft| DraftInfo {
            draft_id: draft.id,
            title: draft.title,
            thumbnail_image: draft.thumbnail_image,
            summary: draft.summary,
            content: draft.content,
            slug: draft.slug,
            created_at: draft.created_at,
            updated_at: draft.updated_at,
        })
        .collect();

    Ok(GetDraftsResponse {
        drafts: draft_infos,
    })
}
