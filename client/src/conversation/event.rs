use chrono::{DateTime, Utc};
use dashmap::mapref::one::RefMut;
use shared::schema::conversation::{
    ConversationMember, ConversationMemberChange, ConversationMemberChangeSerial,
};
use shared::schema::conversation_message::{ConversationMessage, ConversationMessageSerial};
use std::sync::RwLockWriteGuard;
use uuid::Uuid;

use super::panel::ChatPanel;
use super::state::{ConversationEvent, ConversationEventKey, Conversation};
use crate::account;
use crate::state::STATE;

pub fn handle_message(message_serial: ConversationMessageSerial) {
    let conversation_id: Uuid = message_serial.conversation_id;
    let sender_account_id: Uuid = message_serial.sender_account_id;
    let message: ConversationMessage = ConversationMessage::from(message_serial);
    let event: ConversationEvent = ConversationEvent::Chat(message);
    insert_event(conversation_id, event);
    snap_conversation_view_to_bottom_if_at_bottom(conversation_id);
    account::catchup::spawn_fetch_if_missing(sender_account_id);
}

pub fn handle_member_joined(change_serial: ConversationMemberChangeSerial) {
    let conversation_id: Uuid = change_serial.conversation_id;
    let change: ConversationMemberChange = ConversationMemberChange::from(&change_serial);
    let event: ConversationEvent = ConversationEvent::MemberJoined(change);
    insert_event(conversation_id, event);

    STATE.conversation.get_or_create(conversation_id)
        .members
        .entry(change_serial.account_id)
        .or_insert_with(|| ConversationMember::from(&change_serial));

    account::catchup::spawn_fetch_if_missing(change_serial.account_id);
}

/// If the user is currently scrolled to the bottom of this conversation's view, mark the
/// scroll region to land at the new bottom on next draw. Called after inserting an event
/// so newly-arrived messages remain visible without disrupting users who scrolled up to
/// read history.
fn snap_conversation_view_to_bottom_if_at_bottom(conversation_id: Uuid) {
    let mut chat_panel: RwLockWriteGuard<ChatPanel> = STATE.conversation.chat_panel.write().unwrap();
    let Some(view_state) = chat_panel.conversation_view_states.get_mut(&conversation_id) else {
        return;
    };

    if view_state.scroll_region.is_at_bottom() {
        view_state.scroll_region.scroll_to_bottom();
    }
}

pub fn handle_member_left(change_serial: ConversationMemberChangeSerial) {
    let conversation_id: Uuid = change_serial.conversation_id;
    let change: ConversationMemberChange = ConversationMemberChange::from(&change_serial);
    let event: ConversationEvent = ConversationEvent::MemberLeft(change);
    insert_event(conversation_id, event);

    if let Some(mut conversation_entry) = STATE.conversation.conversations.get_mut(&conversation_id) {
        conversation_entry.members.remove(&change_serial.account_id);
    }
}

fn insert_event(conversation_id: Uuid, event: ConversationEvent) {
    let event_key: ConversationEventKey = ConversationEventKey::from(&event);
    let event_timestamp: DateTime<Utc> = event_key.timestamp;

    let mut conversation_entry: RefMut<Uuid, Conversation> = STATE.conversation.get_or_create(conversation_id);

    let previous_value: Option<ConversationEvent> = conversation_entry.events.insert(event_key, event);

    if previous_value.is_none() {
        let is_unread: bool = conversation_entry
            .last_read
            .map_or(true, |last_read| event_timestamp > last_read);
        conversation_entry.unread_count += u32::from(is_unread);
    }
}
