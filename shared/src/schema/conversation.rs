use chrono::{DateTime, Utc};
use raylib::color::Color;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMemberChangeSerial {
    pub conversation_id: Uuid,
    pub account_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

impl From<&ConversationMemberSerial> for ConversationMemberChangeSerial {
    fn from(member: &ConversationMemberSerial) -> Self {
        ConversationMemberChangeSerial {
            conversation_id: member.conversation_id,
            account_id: member.account_id,
            timestamp: member.entered,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConversationMemberChange {
    pub conversation_id: Uuid,
    pub account_id: Uuid,
    pub timestamp: DateTime<Utc>,
}

impl From<&ConversationMemberChangeSerial> for ConversationMemberChange {
    fn from(serial: &ConversationMemberChangeSerial) -> Self {
        ConversationMemberChange {
            conversation_id: serial.conversation_id,
            account_id: serial.account_id,
            timestamp: serial.timestamp,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConversationMember {
    pub conversation_id: Uuid,
    pub account_id: Uuid,
    pub entered: DateTime<Utc>,
    pub exited: Option<DateTime<Utc>>,
    pub color_cached: Option<Color>,
}

impl From<ConversationMemberSerial> for ConversationMember {
    fn from(serial: ConversationMemberSerial) -> Self {
        ConversationMember {
            conversation_id: serial.conversation_id,
            account_id: serial.account_id,
            entered: serial.entered,
            exited: serial.exited,
            color_cached: None,
        }
    }
}
