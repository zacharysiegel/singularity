use shared::schema::conversation::{
    ConversationMember, ConversationMemberChange, ConversationMemberChangeSerial,
    ConversationMemberSerial, ConversationSerial,
};
use shared::schema::conversation_message::{ConversationMessage, ConversationMessageSerial};
use std::collections::HashMap;
use std::sync::RwLockWriteGuard;
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

    let mut conversations: RwLockWriteGuard<HashMap<Uuid, ConversationLog>> =
        STATE.conversation.conversations.write().unwrap();
    if let Some(conversation_log) = conversations.get_mut(&conversation_id) {
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
}

pub fn handle_member_left(change_serial: ConversationMemberChangeSerial) {
    let conversation_id: Uuid = change_serial.conversation_id;
    let change: ConversationMemberChange = ConversationMemberChange::from(change_serial.clone());
    let event: ConversationEvent = ConversationEvent::MemberLeft(change);
    insert_event(conversation_id, event);

    let mut conversations: RwLockWriteGuard<HashMap<Uuid, ConversationLog>> =
        STATE.conversation.conversations.write().unwrap();
    if let Some(conversation_log) = conversations.get_mut(&conversation_id) {
        conversation_log
            .members
            .retain(|member| member.account_id != change_serial.account_id);
    }
}

pub fn store_conversation_metadata(conversation_serial: &ConversationSerial) {
    let mut conversations: RwLockWriteGuard<HashMap<Uuid, ConversationLog>> =
        STATE.conversation.conversations.write().unwrap();
    let conversation_log: &mut ConversationLog = conversations
        .entry(conversation_serial.id)
        .or_insert_with(ConversationLog::new);
    conversation_log.name = conversation_serial.name.clone();
    conversation_log.game_id = conversation_serial.game_id;
    conversation_log.created = Some(conversation_serial.created);
}

pub fn store_conversation_members(
    conversation_id: Uuid,
    member_serials: Vec<ConversationMemberSerial>,
) {
    let mut conversations: RwLockWriteGuard<HashMap<Uuid, ConversationLog>> =
        STATE.conversation.conversations.write().unwrap();
    let conversation_log: &mut ConversationLog = conversations
        .entry(conversation_id)
        .or_insert_with(ConversationLog::new);
    conversation_log.members = member_serials
        .into_iter()
        .map(ConversationMember::from)
        .collect();
}

fn insert_event(conversation_id: Uuid, event: ConversationEvent) {
    let event_key: ConversationEventKey = ConversationEventKey::from(&event);
    let event_timestamp: chrono::DateTime<chrono::Utc> = event_key.timestamp;

    let mut conversations: RwLockWriteGuard<HashMap<Uuid, ConversationLog>> = STATE.conversation.conversations.write().unwrap();
    let conversation_log: &mut ConversationLog = conversations
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
