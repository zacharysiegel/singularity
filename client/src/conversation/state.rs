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
    /// Keyed by account_id for O(1) deduplication and removal. BTreeMap maintains sorted order
    /// by UUID, providing deterministic iteration for donut ring icon segment rendering.
    pub members: BTreeMap<Uuid, ConversationMember>,
    /// Events are stored in a BTreeMap keyed by (timestamp, account_id) so they are always sorted
    /// chronologically. This avoids re-sorting on every render and provides natural deduplication
    /// when catching up via REST (inserting an already-present key is a no-op).
    /// The secondary account_id key prevents duplicate timestamp collision.
    pub events: BTreeMap<ConversationEventKey, ConversationEvent>,
    pub last_read: Option<DateTime<Utc>>,
    pub unread_count: u32,
}

use shared::schema::conversation::ConversationSerial;

impl Conversation {
    pub fn new() -> Self {
        Conversation {
            name: None,
            game_id: None,
            created: None,
            members: BTreeMap::new(),
            events: BTreeMap::new(),
            last_read: None,
            unread_count: 0,
        }
    }

    pub fn set_metadata(&mut self, serial: &ConversationSerial) {
        self.name = serial.name.clone();
        self.game_id = serial.game_id;
        self.created = Some(serial.created);
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
