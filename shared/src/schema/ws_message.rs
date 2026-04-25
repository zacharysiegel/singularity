//! WebSocket message types for lobby and live connections.
//! Requests are sent from the client to the server.
//! Events are sent from the server to the client.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsRequest {
    Chat {
        conversation_id: Uuid,
        content: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsEvent {
    Chat {
        id: Uuid,
        conversation_id: Uuid,
        sender_account_id: Uuid,
        content: String,
        created: DateTime<Utc>,
    },
    MemberJoined {
        conversation_id: Uuid,
        account_id: Uuid,
    },
    MemberLeft {
        conversation_id: Uuid,
        account_id: Uuid,
    },
    Error {
        message: String,
    },
}
