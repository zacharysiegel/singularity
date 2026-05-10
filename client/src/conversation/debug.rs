use crate::conversation::state::Conversation;
use crate::state::STATE;
use chrono::Utc;
use shared::random;
use uuid::Uuid;

pub fn seed_debug_conversations() {
    let conversation_ids: [Uuid; 4] = [
        random::random_uuid(),
        random::random_uuid(),
        random::random_uuid(),
        random::random_uuid(),
    ];

    let mut conversation_a: Conversation = Conversation::new();
    conversation_a.name = Some("Strategy Squad".to_string());
    conversation_a.created = Some(Utc::now());
    conversation_a.unread_count = 3;
    STATE.conversation.conversations.insert(conversation_ids[0], conversation_a);

    let mut conversation_b: Conversation = Conversation::new();
    conversation_b.name = Some("Game: Expansion".to_string());
    conversation_b.created = Some(Utc::now());
    conversation_b.unread_count = 12;
    STATE.conversation.conversations.insert(conversation_ids[1], conversation_b);

    let mut conversation_c: Conversation = Conversation::new();
    conversation_c.name = Some("player_alpha".to_string());
    conversation_c.created = Some(Utc::now());
    STATE.conversation.conversations.insert(conversation_ids[2], conversation_c);

    let mut conversation_d: Conversation = Conversation::new();
    conversation_d.name = Some("A very long conversation name that should get truncated by the ellipsis".to_string());
    conversation_d.created = Some(Utc::now());
    conversation_d.unread_count = 1;
    STATE.conversation.conversations.insert(conversation_ids[3], conversation_d);

    log::info!("Seeded {} debug conversations", conversation_ids.len());
}
