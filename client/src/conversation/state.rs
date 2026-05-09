use chrono::{DateTime, Utc};
use dashmap::DashMap;
use shared::schema::conversation::{ConversationMember, ConversationMemberChange};
use shared::schema::conversation_message::ConversationMessage;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct ConversationState {
    pub conversations: DashMap<Uuid, Conversation>,
}

impl ConversationState {
    pub fn new() -> Self {
        ConversationState {
            conversations: DashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Conversation {
    pub name: Option<String>,
    pub game_id: Option<Uuid>,
    pub created: Option<DateTime<Utc>>,
    pub members: Vec<ConversationMember>,
    /// Events are stored in a BTreeMap keyed by (timestamp, account_id) so they are always sorted
    /// chronologically. This avoids re-sorting on every render and provides natural deduplication
    /// when catching up via REST (inserting an already-present key is a no-op).
    /// The secondary account_id key prevents duplicate timestamp collision.
    pub events: BTreeMap<ConversationEventKey, ConversationEvent>,
    pub last_read: Option<DateTime<Utc>>,
    pub unread_count: u32,
}

impl Conversation {
    pub fn new() -> Self {
        Conversation {
            name: None,
            game_id: None,
            created: None,
            members: Vec::new(),
            events: BTreeMap::new(),
            last_read: None,
            unread_count: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConversationEventKey {
    pub timestamp: DateTime<Utc>,
    pub account_id: Uuid,
}

impl From<&ConversationEvent> for ConversationEventKey {
    fn from(value: &ConversationEvent) -> Self {
        match value {
            ConversationEvent::Chat(message) => ConversationEventKey {
                timestamp: message.created,
                account_id: message.sender_account_id,
            },
            ConversationEvent::MemberJoined(change) => ConversationEventKey {
                timestamp: change.timestamp,
                account_id: change.account_id,
            },
            ConversationEvent::MemberLeft(change) => ConversationEventKey {
                timestamp: change.timestamp,
                account_id: change.account_id,
            },
        }
    }
}

#[derive(Debug)]
pub enum ConversationEvent {
    Chat(ConversationMessage),
    MemberJoined(ConversationMemberChange),
    MemberLeft(ConversationMemberChange),
}
