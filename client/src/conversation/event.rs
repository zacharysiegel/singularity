use shared::schema::conversation::{
    ConversationMember, ConversationMemberChange, ConversationMemberChangeSerial,
    ConversationMemberSerial,
};
use shared::schema::conversation_message::{ConversationMessage, ConversationMessageSerial};
use uuid::Uuid;

use super::state::{ConversationEvent, ConversationEventKey, ConversationLog};
use crate::state::STATE;

pub fn handle_chat_event(message_serial: ConversationMessageSerial) {
    let conversation_id: Uuid = message_serial.conversation_id;
    let message: ConversationMessage = ConversationMessage::from(message_serial);
    let event: ConversationEvent = ConversationEvent::Chat(message);
    insert_event(conversation_id, event);
}

pub fn handle_member_joined(change_serial: ConversationMemberChangeSerial) {
    let conversation_id: Uuid = change_serial.conversation_id;
    let change: ConversationMemberChange = ConversationMemberChange::from(change_serial.clone());
    let event: ConversationEvent = ConversationEvent::MemberJoined(change);
    insert_event(conversation_id, event);

    let mut conversation_log = STATE.conversation.conversations
        .entry(conversation_id)
        .or_insert_with(ConversationLog::new);
    let already_member: bool = conversation_log
        .members
        .iter()
        .any(|member| member.account_id == change_serial.account_id);
    if !already_member {
        conversation_log.members.push(ConversationMember::from(
            ConversationMemberSerial {
                conversation_id: change_serial.conversation_id,
                account_id: change_serial.account_id,
                entered: change_serial.timestamp,
                exited: None,
            },
        ));
    }
}

pub fn handle_member_left(change_serial: ConversationMemberChangeSerial) {
    let conversation_id: Uuid = change_serial.conversation_id;
    let change: ConversationMemberChange = ConversationMemberChange::from(change_serial.clone());
    let event: ConversationEvent = ConversationEvent::MemberLeft(change);
    insert_event(conversation_id, event);

    if let Some(mut conversation_log) = STATE.conversation.conversations.get_mut(&conversation_id) {
        conversation_log
            .members
            .retain(|member| member.account_id != change_serial.account_id);
    }
}

fn insert_event(conversation_id: Uuid, event: ConversationEvent) {
    let event_key: ConversationEventKey = ConversationEventKey::from(&event);
    let event_timestamp: chrono::DateTime<chrono::Utc> = event_key.timestamp;

    let mut conversation_log = STATE.conversation.conversations
        .entry(conversation_id)
        .or_insert_with(ConversationLog::new);

    let is_new: bool = !conversation_log.events.contains_key(&event_key);
    conversation_log.events.insert(event_key, event);

    if is_new {
        let is_unread: bool = conversation_log
            .last_read
            .map_or(true, |last_read| event_timestamp > last_read);
        if is_unread {
            conversation_log.unread_count += 1;
        }
    }
}
