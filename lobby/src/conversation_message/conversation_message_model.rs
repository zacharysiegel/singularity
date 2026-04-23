use chrono::{DateTime, Utc};
use shared::schema::conversation_message::ConversationMessageSerial;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ConversationMessage {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_account_id: Uuid,
    pub content: String,
    pub sender_anonymized: bool,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ConversationMessageRow {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_account_id: Uuid,
    pub content: String,
    pub sender_anonymized: bool,
    pub created: DateTime<Utc>,
}

impl From<&ConversationMessageRow> for ConversationMessageSerial {
    fn from(row: &ConversationMessageRow) -> Self {
        ConversationMessageSerial {
            id: row.id,
            conversation_id: row.conversation_id,
            sender_account_id: row.sender_account_id,
            content: row.content.clone(),
            sender_anonymized: row.sender_anonymized,
            created: row.created,
        }
    }
}
