use chrono::{DateTime, Utc};
use dashmap::mapref::one::RefMut;
use shared::schema::conversation::{
    ConversationMember, ConversationMemberChange, ConversationMemberChangeSerial,
};
use shared::schema::conversation_message::{ConversationMessage, ConversationMessageSerial};
use uuid::Uuid;

use super::state::{ConversationEvent, ConversationEventKey, Conversation};
use crate::state::STATE;

pub fn handle_message(message_serial: ConversationMessageSerial) {
    let conversation_id: Uuid = message_serial.conversation_id;
    let message: ConversationMessage = ConversationMessage::from(message_serial);
    let event: ConversationEvent = ConversationEvent::Chat(message);
    insert_event(conversation_id, event);
}

pub fn handle_member_joined(change_serial: ConversationMemberChangeSerial) {
    let conversation_id: Uuid = change_serial.conversation_id;
    let change: ConversationMemberChange = ConversationMemberChange::from(&change_serial);
    let event: ConversationEvent = ConversationEvent::MemberJoined(change);
    insert_event(conversation_id, event);

    let mut conversation_entry: RefMut<Uuid, Conversation> = STATE.conversation.conversations
        .entry(conversation_id)
        .or_insert_with(|| {
            STATE.conversation.mark_display_order_dirty();
            Conversation::new()
        });
    conversation_entry.members
        .entry(change_serial.account_id)
        .or_insert_with(|| ConversationMember::from(&change_serial));
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

    let mut conversation_entry: RefMut<Uuid, Conversation> = STATE.conversation.conversations
        .entry(conversation_id)
        .or_insert_with(|| {
            STATE.conversation.mark_display_order_dirty();
            Conversation::new()
        });

    let previous_value: Option<ConversationEvent> = conversation_entry.events.insert(event_key, event);

    if previous_value.is_none() {
        let is_unread: bool = conversation_entry
            .last_read
            .map_or(true, |last_read| event_timestamp > last_read);
        conversation_entry.unread_count += u32::from(is_unread);
    }
}
