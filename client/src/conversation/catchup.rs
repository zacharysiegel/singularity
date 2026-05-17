use shared::error::AppErrorStatic;
use shared::schema::conversation::{ConversationMemberChangeSerial, ConversationMemberSerial, ConversationSerial};
use shared::schema::conversation_message::ConversationMessageSerial;
use uuid::Uuid;

use super::api;
use super::event;
use crate::account;
use crate::state::STATE;

pub async fn catch_up(token: &str) {
    let conversation_serials: Vec<ConversationSerial> = match api::get_conversations(token).await {
        Ok(conversations) => conversations,
        Err(error) => {
            log::warn!("Chat catch-up failed to fetch conversations; [{error}]");
            return;
        }
    };

    let per_conversation_message_counts: Vec<usize> = futures::future::join_all(
        conversation_serials
            .iter()
            .map(|conversation_serial| catch_up_conversation(token, conversation_serial)),
    )
    .await;

    let conversation_with_messages_count: usize = per_conversation_message_counts
        .iter()
        .filter(|count| **count > 0)
        .count();
    let message_count: usize = per_conversation_message_counts.iter().sum();

    log::info!("Chat catch-up complete; [{conversation_with_messages_count} conversations] [{message_count} messages]");
}

/// Fetches members and messages for a single conversation concurrently, applies them via
/// the same event path as live WS, and resolves any uncached member usernames. Returns the
/// number of messages applied.
async fn catch_up_conversation(token: &str, conversation_serial: &ConversationSerial) -> usize {
    STATE.conversation.get_or_create(conversation_serial.id)
        .set_metadata(conversation_serial);

    let (member_result, message_result): (
        Result<Vec<ConversationMemberSerial>, AppErrorStatic>,
        Result<Vec<ConversationMessageSerial>, AppErrorStatic>,
    ) = tokio::join!(
        api::get_members(token, conversation_serial.id),
        api::get_messages(token, conversation_serial.id),
    );

    let member_serials: Vec<ConversationMemberSerial> = match member_result {
        Ok(members) => members,
        Err(error) => {
            log::warn!(
                "Chat catch-up failed to fetch members; [{}] [{error}]",
                conversation_serial.id
            );
            Vec::new()
        }
    };
    for member_serial in &member_serials {
        event::handle_member_joined(ConversationMemberChangeSerial::from(member_serial));
    }
    let member_account_ids: Vec<Uuid> = member_serials
        .iter()
        .map(|member_serial| member_serial.account_id)
        .collect();
    account::catchup::fetch_missing_accounts(token, &member_account_ids).await;

    let message_serials: Vec<ConversationMessageSerial> = match message_result {
        Ok(messages) => messages,
        Err(error) => {
            log::warn!(
                "Chat catch-up failed to fetch messages; [{}] [{error}]",
                conversation_serial.id
            );
            Vec::new()
        }
    };
    let message_count: usize = message_serials.len();
    for message_serial in message_serials {
        event::handle_message(message_serial);
    }
    message_count
}
