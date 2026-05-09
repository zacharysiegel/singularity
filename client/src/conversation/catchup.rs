use shared::environment::RuntimeEnvironment;
use shared::error::AppErrorStatic;
use shared::schema::conversation::{ConversationMemberChangeSerial, ConversationMemberSerial, ConversationSerial};
use shared::schema::conversation_message::ConversationMessageSerial;
use std::collections::HashMap;
use std::sync::RwLockWriteGuard;
use uuid::Uuid;

use super::event;
use super::state::ConversationLog;
use crate::http;
use crate::state::STATE;

const MESSAGE_LIMIT: i64 = 64;

pub async fn catch_up(token: &str) {
    let conversation_serials: Vec<ConversationSerial> = match fetch_conversations(token).await {
        Ok(conversations) => conversations,
        Err(error) => {
            log::warn!("Chat catch-up failed to fetch conversations; [{error}]");
            return;
        }
    };

    let mut conversation_count: i32 = 0;
    let mut message_count: i32 = 0;

    for conversation_serial in &conversation_serials {
        store_conversation_metadata(conversation_serial);

        let member_serials: Vec<ConversationMemberSerial> =
            match fetch_members(token, conversation_serial.id).await {
                Ok(members) => members,
                Err(error) => {
                    log::warn!(
                        "Chat catch-up failed to fetch members; [{}] [{error}]",
                        conversation_serial.id
                    );
                    Vec::new()
                }
            };
        for member_serial in member_serials {
            event::handle_member_joined(ConversationMemberChangeSerial {
                conversation_id: member_serial.conversation_id,
                account_id: member_serial.account_id,
                timestamp: member_serial.entered,
            });
        }

        let message_serials: Vec<ConversationMessageSerial> =
            match fetch_messages(token, conversation_serial.id).await {
                Ok(messages) => messages,
                Err(error) => {
                    log::warn!(
                        "Chat catch-up failed to fetch messages; [{}] [{error}]",
                        conversation_serial.id
                    );
                    continue;
                }
            };

        conversation_count += i32::from(!message_serials.is_empty());
        message_count += message_serials.len() as i32;
        for message_serial in message_serials {
            event::handle_chat_event(message_serial);
        }
    }

    log::info!("Chat catch-up complete; [{conversation_count} conversations] [{message_count} messages]");
}

async fn fetch_conversations(token: &str) -> Result<Vec<ConversationSerial>, AppErrorStatic> {
    let lobby_http_origin: String = RuntimeEnvironment::default().lobby_http_origin();
    let url: String = format!("{lobby_http_origin}/conversation");
    http::fetch_standard(token, &url, "conversations").await
}

async fn fetch_messages(
    token: &str,
    conversation_id: Uuid,
) -> Result<Vec<ConversationMessageSerial>, AppErrorStatic> {
    let lobby_http_origin: String = RuntimeEnvironment::default().lobby_http_origin();
    let url: String = format!(
        "{lobby_http_origin}/conversation/{conversation_id}/message?limit={MESSAGE_LIMIT}"
    );
    http::fetch_standard(token, &url, &format!("messages; [{conversation_id}]")).await
}

async fn fetch_members(
    token: &str,
    conversation_id: Uuid,
) -> Result<Vec<ConversationMemberSerial>, AppErrorStatic> {
    let lobby_http_origin: String = RuntimeEnvironment::default().lobby_http_origin();
    let url: String = format!("{lobby_http_origin}/conversation/{conversation_id}/member");
    http::fetch_standard(token, &url, &format!("members; [{conversation_id}]")).await
}

fn store_conversation_metadata(conversation_serial: &ConversationSerial) {
    let mut conversations: RwLockWriteGuard<HashMap<Uuid, ConversationLog>> =
        STATE.conversation.conversations.write().unwrap();
    let conversation_log: &mut ConversationLog = conversations
        .entry(conversation_serial.id)
        .or_insert_with(ConversationLog::new);
    conversation_log.name = conversation_serial.name.clone();
    conversation_log.game_id = conversation_serial.game_id;
    conversation_log.created = Some(conversation_serial.created);
}
