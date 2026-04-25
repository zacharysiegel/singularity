use chrono::{DateTime, Utc};
use shared::schema::conversation_message::ConversationMessageSerial;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ConversationMessageEntity {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub sender_account_id: Uuid,
    pub content: String,
    pub created: DateTime<Utc>,
}

impl From<&ConversationMessageEntity> for ConversationMessageSerial {
    fn from(entity: &ConversationMessageEntity) -> Self {
        ConversationMessageSerial {
            id: entity.id,
            conversation_id: entity.conversation_id,
            sender_account_id: entity.sender_account_id,
            content: entity.content.clone(),
            created: entity.created,
        }
    }
}
