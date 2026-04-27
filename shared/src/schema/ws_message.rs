//! WebSocket message types for lobby and live connections.
//! Requests are sent from the client to the server.
//! Events are sent from the server to the client.

use super::conversation_message::ConversationMessageSerial;
use crate::schema::conversation::ConversationMemberChangeSerial;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum ConnectionType {
    Live,
    Lobby,
}

impl Display for ConnectionType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let connection_type_name: &str = match self {
            ConnectionType::Live => "live",
            ConnectionType::Lobby => "lobby",
        };
        write!(formatter, "{}", connection_type_name)
    }
}

impl ConnectionType {
    pub fn ws_path(&self) -> String {
        format!("/ws/{}", self.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsRequest {
    Chat { conversation_id: Uuid, content: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsEvent {
    Chat(ConversationMessageSerial),
    MemberJoined(ConversationMemberChangeSerial),
    MemberLeft(ConversationMemberChangeSerial),
    Error { message: String },
}
