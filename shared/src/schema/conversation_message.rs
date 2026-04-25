use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessageSerial {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_account_id: Uuid,
    pub content: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SendMessageRequest {
    pub conversation_id: Uuid,
    pub content: String,
}

impl SendMessageRequest {
    pub fn is_valid(&self) -> bool {
        !self.content.is_empty()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConversationMessageQuery {
    pub limit: Option<i64>,
    pub before: Option<DateTime<Utc>>,
}
