use shared::schema::conversation::{ConversationMemberChange, ConversationMemberChangeSerial};
use shared::schema::conversation_message::{ConversationMessage, ConversationMessageSerial};
use uuid::Uuid;

use crate::state::STATE;
use super::state::{ConversationEvent, ConversationLog};

pub fn handle_chat_event(message_serial: ConversationMessageSerial) {
    let conversation_id: Uuid = message_serial.conversation_id;
    let message: ConversationMessage = ConversationMessage::from(message_serial);
    let event: ConversationEvent = ConversationEvent::Chat(message);
    insert_event(conversation_id, event);
}

pub fn handle_member_joined(change_serial: ConversationMemberChangeSerial) {
    let conversation_id: Uuid = change_serial.conversation_id;
    let change: ConversationMemberChange = ConversationMemberChange::from(change_serial);
    let event: ConversationEvent = ConversationEvent::MemberJoined(change);
    insert_event(conversation_id, event);
}

pub fn handle_member_left(change_serial: ConversationMemberChangeSerial) {
    let conversation_id: Uuid = change_serial.conversation_id;
    let change: ConversationMemberChange = ConversationMemberChange::from(change_serial);
    let event: ConversationEvent = ConversationEvent::MemberLeft(change);
    insert_event(conversation_id, event);
}

fn insert_event(conversation_id: Uuid, event: ConversationEvent) {
    let mut conversations = STATE.conversation.conversations.write().unwrap();
    let conversation_log: &mut ConversationLog = conversations
        .entry(conversation_id)
        .or_insert_with(ConversationLog::new);
    conversation_log.events.push(event);
}
