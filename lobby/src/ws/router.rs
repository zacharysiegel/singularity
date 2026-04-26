use std::sync::Arc;
use shared::error::AppError;
use shared::schema::ws_message::{WsRequest, WsEvent};
use sqlx::PgPool;
use uuid::Uuid;

use crate::conversation::conversation_db;
use crate::conversation::conversation_model::ConversationMemberEntity;
use crate::conversation_message::conversation_message_db;
use crate::conversation_message::conversation_message_model::ConversationMessageEntity;
use super::connection_registry;
use super::connection_type::ConnectionType;

pub async fn route_ws_request(
    pool: &PgPool,
    connection_type: ConnectionType,
    sender_account_id: Uuid,
    ws_request: WsRequest,
) -> Result<(), AppError> {
    match ws_request {
        WsRequest::Chat { conversation_id, content } => {
            handle_chat(pool, connection_type, sender_account_id, conversation_id, &content).await
        }
    }
}

async fn handle_chat(
    pool: &PgPool,
    connection_type: ConnectionType,
    sender_account_id: Uuid,
    conversation_id: Uuid,
    content: &str,
) -> Result<(), AppError> {
    let member: Option<ConversationMemberEntity> = conversation_db::get_member(pool, conversation_id, sender_account_id).await?;
    if member.is_none() {
        return Err(AppError::new("not a member of this conversation"));
    }

    let message_entity: ConversationMessageEntity = conversation_message_db::create_message(
        pool,
        conversation_id,
        sender_account_id,
        content,
    ).await?;

    let ws_event: WsEvent = message_entity.to_ws_event();
    let member_ids: Vec<Uuid> = conversation_db::get_active_member_ids(pool, conversation_id).await?;

    connection_registry::send_to_accounts(&member_ids, connection_type, &Arc::new(ws_event));
    Ok(())
}
