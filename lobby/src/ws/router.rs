use shared::error::AppError;
use shared::schema::ws_message::{InboundMessage, OutboundMessage};
use sqlx::PgPool;
use uuid::Uuid;

use crate::conversation::conversation_db;
use crate::conversation_message::conversation_message_db;
use super::connection_registry;

pub async fn handle_inbound_message(
    pool: &PgPool,
    sender_account_id: Uuid,
    inbound: InboundMessage,
) -> Result<(), AppError> {
    match inbound {
        InboundMessage::SendMessage { conversation_id, content } => {
            handle_send_message(pool, sender_account_id, conversation_id, &content).await
        }
    }
}

async fn handle_send_message(
    pool: &PgPool,
    sender_account_id: Uuid,
    conversation_id: Uuid,
    content: &str,
) -> Result<(), AppError> {
    let member = conversation_db::get_member(pool, conversation_id, sender_account_id).await?;
    if member.is_none() {
        return Err(AppError::new("not a member of this conversation"));
    }

    let message_row = conversation_message_db::create_message(
        pool,
        conversation_id,
        sender_account_id,
        content,
    )
    .await?;

    let outbound: OutboundMessage = OutboundMessage::ReceiveMessage {
        id: message_row.id,
        conversation_id: message_row.conversation_id,
        sender_account_id: message_row.sender_account_id,
        content: message_row.content,
        created: message_row.created,
    };

    let member_ids: Vec<Uuid> = conversation_db::get_active_member_ids(pool, conversation_id).await?;
    connection_registry::send_to_accounts(&member_ids, &outbound);

    Ok(())
}
