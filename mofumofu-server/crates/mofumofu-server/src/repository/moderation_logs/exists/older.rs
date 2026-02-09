use super::super::find_list::ModerationLogFilter;
use mofumofu_entity::moderation_logs::{
    Column as ModerationLogColumn, Entity as ModerationLogEntity,
};
use mofumofu_errors::errors::Errors;
use sea_orm::{
    ColumnTrait, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use uuid::Uuid;

pub async fn repository_exists_older_moderation_log<C>(
    conn: &C,
    filter: &ModerationLogFilter,
    cursor_id: Uuid,
) -> Result<bool, Errors>
where
    C: ConnectionTrait,
{
    let mut query = ModerationLogEntity::find().filter(ModerationLogColumn::Id.lt(cursor_id));

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

    let count = query.limit(1).count(conn).await?;
    Ok(count > 0)
}
