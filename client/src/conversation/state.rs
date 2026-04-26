use shared::schema::conversation::ConversationMemberChange;
use shared::schema::conversation_message::ConversationMessage;
use std::collections::HashMap;
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
    pub events: Vec<ConversationEvent>,
}

impl ConversationLog {
    pub fn new() -> Self {
        ConversationLog {
            events: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum ConversationEvent {
    Chat(ConversationMessage),
    MemberJoined(ConversationMemberChange),
    MemberLeft(ConversationMemberChange),
}
