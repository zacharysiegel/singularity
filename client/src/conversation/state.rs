use chrono::{DateTime, Utc};
use shared::schema::conversation::ConversationMemberChange;
use shared::schema::conversation_message::ConversationMessage;
use std::collections::{BTreeMap, HashMap};
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
pub struct ConversationState {
    pub conversations: RwLock<HashMap<Uuid, ConversationLog>>,
}

impl ConversationState {
    pub fn new() -> Self {
        ConversationState {
            conversations: RwLock::new(HashMap::new()),
        }
    }
}

#[derive(Debug)]
pub struct ConversationLog {
    /// Events are stored in a BTreeMap keyed by (timestamp, account_id) so they are always sorted
    /// chronologically. This avoids re-sorting on every render and provides natural deduplication
    /// when catching up via REST (inserting an already-present key is a no-op).
    /// The secondary account_id key prevents duplicate timestamp collision.
    pub events: BTreeMap<ConversationEventKey, ConversationEvent>,
}

impl ConversationLog {
    pub fn new() -> Self {
        ConversationLog {
            events: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConversationEventKey {
    pub timestamp: DateTime<Utc>,
    pub account_id: Uuid,
}

#[derive(Debug)]
pub enum ConversationEvent {
    Chat(ConversationMessage),
    MemberJoined(ConversationMemberChange),
    MemberLeft(ConversationMemberChange),
}

impl ConversationEvent {
    pub fn key(&self) -> ConversationEventKey {
        match self {
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
