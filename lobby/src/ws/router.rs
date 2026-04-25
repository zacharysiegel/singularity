use shared::error::AppError;
use shared::schema::ws_message::{WsRequest, WsEvent};
use sqlx::PgPool;
use uuid::Uuid;

use crate::conversation::conversation_db;
use crate::conversation_message::conversation_message_db;
use super::connection_registry;
use super::connection_type::ConnectionType;

pub async fn handle_ws_request(
    pool: &PgPool,
    connection_type: ConnectionType,
    sender_account_id: Uuid,
    ws_request: WsRequest,
) -> Result<(), AppError> {
    match ws_request {
        WsRequest::ChatMessage { conversation_id, content } => {
            handle_chat_message(pool, connection_type, sender_account_id, conversation_id, &content).await
        }
    }
}

async fn handle_chat_message(
    pool: &PgPool,
    connection_type: ConnectionType,
    sender_account_id: Uuid,
    conversation_id: Uuid,
    content: &str,
) -> Result<(), AppError> {
    let member = conversation_db::get_member(pool, conversation_id, sender_account_id).await?;
    if member.is_none() {
        return Err(AppError::new("not a member of this conversation"));
    }

    let message_entity = conversation_message_db::create_message(
        pool,
        conversation_id,
        sender_account_id,
        content,
    )
    .await?;

    let ws_event: WsEvent = WsEvent::ChatMessage {
        id: message_entity.id,
        conversation_id: message_entity.conversation_id,
        sender_account_id: message_entity.sender_account_id,
        content: message_entity.content,
        created: message_entity.created,
    };

    let member_ids: Vec<Uuid> = conversation_db::get_active_member_ids(pool, conversation_id).await?;
    connection_registry::send_to_accounts(&member_ids, connection_type, &ws_event);

    Ok(())
}
