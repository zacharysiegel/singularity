use chrono::{DateTime, Utc};
use shared::schema::conversation::{ConversationMemberSerial, ConversationSerial};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub game_id: Option<Uuid>,
    pub name: Option<String>,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct ConversationEntity {
    pub id: Uuid,
    pub game_id: Option<Uuid>,
    pub name: Option<String>,
    pub created: DateTime<Utc>,
}

impl From<ConversationEntity> for Conversation {
    fn from(entity: ConversationEntity) -> Self {
        Conversation {
            id: entity.id,
            game_id: entity.game_id,
            name: entity.name,
            created: entity.created,
        }
    }
}

impl From<&Conversation> for ConversationSerial {
    fn from(model: &Conversation) -> Self {
        ConversationSerial {
            id: model.id,
            game_id: model.game_id,
            name: model.name.clone(),
            created: model.created,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConversationMember {
    pub conversation_id: Uuid,
    pub account_id: Uuid,
    pub entered: DateTime<Utc>,
    pub exited: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct ConversationMemberEntity {
    pub conversation_id: Uuid,
    pub account_id: Uuid,
    pub entered: DateTime<Utc>,
    pub exited: Option<DateTime<Utc>>,
}

impl From<ConversationMemberEntity> for ConversationMember {
    fn from(entity: ConversationMemberEntity) -> Self {
        ConversationMember {
            conversation_id: entity.conversation_id,
            account_id: entity.account_id,
            entered: entity.entered,
            exited: entity.exited,
        }
    }
}

impl From<&ConversationMember> for ConversationMemberSerial {
    fn from(model: &ConversationMember) -> Self {
        ConversationMemberSerial {
            conversation_id: model.conversation_id,
            account_id: model.account_id,
            entered: model.entered,
            exited: model.exited,
        }
    }
}
