use mofumofu_dto::pagination::CursorDirection;
use mofumofu_entity::common::ModerationResourceType;
use mofumofu_entity::moderation_logs::{
    Column as ModerationLogColumn, Entity as ModerationLogEntity, Model as ModerationLogModel,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct ModerationLogFilter {
    pub actor_id: Option<Uuid>,
    pub resource_type: Option<ModerationResourceType>,
    pub resource_id: Option<Uuid>,
    pub action: Option<String>,
}

pub async fn repository_find_moderation_logs<C>(
    conn: &C,
    filter: &ModerationLogFilter,
    cursor_id: Option<Uuid>,
    cursor_direction: Option<CursorDirection>,
    limit: u64,
) -> Result<Vec<ModerationLogModel>, Errors>
where
    C: ConnectionTrait,
{
    let mut query = ModerationLogEntity::find();

    if let Some(actor_id) = filter.actor_id {
        query = query.filter(ModerationLogColumn::ActorId.eq(actor_id));
    }

    if let Some(ref resource_type) = filter.resource_type {
        query = query.filter(ModerationLogColumn::ResourceType.eq(resource_type.clone()));
    }

    if let Some(resource_id) = filter.resource_id {
        query = query.filter(ModerationLogColumn::ResourceId.eq(resource_id));
    }

    if let Some(ref action) = filter.action {
        query = query.filter(ModerationLogColumn::Action.eq(action.clone()));
    }

    if let Some(id) = cursor_id {
        let direction = cursor_direction.unwrap_or(CursorDirection::Older);
        query = match direction {
            CursorDirection::Older => query
                .filter(ModerationLogColumn::Id.lt(id))
                .order_by_desc(ModerationLogColumn::Id),
            CursorDirection::Newer => query
                .filter(ModerationLogColumn::Id.gt(id))
                .order_by_asc(ModerationLogColumn::Id),
        };
    } else {
        query = query.order_by_desc(ModerationLogColumn::Id);
    }

    let logs = query.limit(limit).all(conn).await?;

    Ok(logs)
}
