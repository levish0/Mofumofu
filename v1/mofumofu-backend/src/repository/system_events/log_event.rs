use crate::entity::common::{ActionType, TargetType};
use crate::entity::system_events::{ActiveModel, Entity as SystemEventEntity};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseConnection, Set, TransactionTrait};
use serde_json::Value;
use tracing::{error, info};
use uuid::Uuid;

/// 이벤트 로깅 - 실패해도 메인 비즈니스 로직에 영향을 주지 않음
pub async fn repository_log_event<C>(
    conn: &C,
    user_id: Option<Uuid>,
    action_type: ActionType,
    target_id: Option<Uuid>,
    target_type: Option<TargetType>,
    metadata: Option<Value>,
) where
    C: ConnectionTrait,
{
    let event = ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id),
        action_type: Set(action_type.clone()),
        target_id: Set(target_id),
        target_type: Set(target_type.clone()),
        metadata: Set(metadata.clone()),
        created_at: Set(chrono::Utc::now()),
    };

    match event.insert(conn).await {
        Ok(_) => {
            info!("Event logged: {:?} for user {:?}", action_type, user_id);
        }
        Err(e) => {
            error!(
                "Failed to log event {:?} for user {:?}: {}",
                action_type, user_id, e
            );
        }
    }
}
