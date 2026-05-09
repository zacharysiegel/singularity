use shared::environment::RuntimeEnvironment;
use shared::error::AppErrorStatic;
use shared::schema::conversation::{ConversationMemberSerial, ConversationSerial};
use shared::schema::conversation_message::ConversationMessageSerial;
use uuid::Uuid;

use super::event;
use crate::http;

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
        event::store_conversation_metadata(conversation_serial);

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
        event::store_conversation_members(conversation_serial.id, member_serials);

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
