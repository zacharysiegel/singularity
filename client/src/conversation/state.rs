use chrono::{DateTime, Utc};
use dashmap::DashMap;
use dashmap::mapref::one::RefMut;
use shared::schema::conversation::{ConversationMember, ConversationMemberChange};
use shared::schema::conversation_message::ConversationMessage;
use std::collections::BTreeMap;
use std::sync::RwLock;
use uuid::Uuid;

use super::panel::ChatPanel;

#[derive(Debug)]
pub struct ConversationState {
    pub conversations: DashMap<Uuid, Conversation>,
    pub chat_panel: RwLock<ChatPanel>,
    pub display_order: RwLock<Vec<Uuid>>,
    display_order_dirty: RwLock<bool>,
}

impl ConversationState {
    pub fn new() -> Self {
        ConversationState {
            conversations: DashMap::new(),
            chat_panel: RwLock::new(ChatPanel::new()),
            display_order: RwLock::new(Vec::new()),
            display_order_dirty: RwLock::new(true),
        }
    }

    pub fn get_or_create(&self, conversation_id: Uuid) -> RefMut<'_, Uuid, Conversation> {
        self.conversations
            .entry(conversation_id)
            .or_insert_with(|| {
                self.mark_display_order_dirty();
                Conversation::new()
            })
    }

    pub fn mark_display_order_dirty(&self) {
        *self.display_order_dirty.write().unwrap() = true;
    }

    pub fn refresh_display_order_if_dirty(&self) {
        let dirty: bool = *self.display_order_dirty.read().unwrap();
        if !dirty {
            return;
        }

        let mut conversation_ids: Vec<Uuid> = self.conversations
            .iter()
            .map(|entry| *entry.key())
            .collect();
        // Deterministic ordering for stable draw/click correspondence (descending = newest first)
        conversation_ids.sort_by(|a, b| b.cmp(a));

        *self.display_order.write().unwrap() = conversation_ids;
        *self.display_order_dirty.write().unwrap() = false;
    }
}

#[derive(Debug)]
pub struct Conversation {
    pub name: Option<String>,
    pub game_id: Option<Uuid>,
    pub created: Option<DateTime<Utc>>,
    /// Keyed by account_id for O(1) deduplication and removal. BTreeMap maintains sorted order
    /// by UUID, providing deterministic iteration member list rendering.
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
