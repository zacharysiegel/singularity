use shared::environment::RuntimeEnvironment;
use shared::error::AppErrorStatic;
use shared::schema::conversation::{ConversationMemberSerial, ConversationSerial};
use shared::schema::conversation_message::ConversationMessageSerial;
use uuid::Uuid;

use crate::http;

const MESSAGE_LIMIT: i64 = 64;

pub async fn get_conversations(token: &str) -> Result<Vec<ConversationSerial>, AppErrorStatic> {
    let lobby_http_origin: String = RuntimeEnvironment::default().lobby_http_origin();
    let url: String = format!("{lobby_http_origin}/conversation");
    http::fetch_standard(token, &url).await
}

pub async fn get_messages(
    token: &str,
    conversation_id: Uuid,
) -> Result<Vec<ConversationMessageSerial>, AppErrorStatic> {
    let lobby_http_origin: String = RuntimeEnvironment::default().lobby_http_origin();
    let url: String = format!(
        "{lobby_http_origin}/conversation/{conversation_id}/message?limit={MESSAGE_LIMIT}"
    );
    http::fetch_standard(token, &url).await
}

pub async fn get_members(
    token: &str,
    conversation_id: Uuid,
) -> Result<Vec<ConversationMemberSerial>, AppErrorStatic> {
    let lobby_http_origin: String = RuntimeEnvironment::default().lobby_http_origin();
    let url: String = format!("{lobby_http_origin}/conversation/{conversation_id}/member");
    http::fetch_standard(token, &url).await
}
