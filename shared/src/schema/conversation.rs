use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSerial {
    pub id: Uuid,
    pub game_id: Option<Uuid>,
    pub name: Option<String>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateConversationRequest {
    pub member_account_ids: Vec<Uuid>,
    pub name: Option<String>,
}

impl CreateConversationRequest {
    pub fn is_valid(&self) -> bool {
        !self.member_account_ids.is_empty()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddConversationMemberRequest {
    pub account_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMemberSerial {
    pub conversation_id: Uuid,
    pub account_id: Uuid,
    pub entered: DateTime<Utc>,
    pub exited: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct ConversationMemberChange {
    pub conversation_id: Uuid,
    pub account_id: Uuid,
    pub timestamp: DateTime<Utc>,
}
