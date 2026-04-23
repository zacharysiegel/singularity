use shared::schema::ws_message::OutboundMessage;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use uuid::Uuid;

pub type OutboundSender = mpsc::UnboundedSender<OutboundMessage>;
pub type OutboundReceiver = mpsc::UnboundedReceiver<OutboundMessage>;

/// Registry of active WebSocket connections, mapping account_id to outbound message senders.
/// Thread-safe via Arc<Mutex>. The senders are Send, so they can be stored in shared state
/// even though the actix_ws::Session they ultimately deliver to is not Send.
#[derive(Debug, Clone)]
pub struct ConnectionRegistry {
    connections: Arc<Mutex<HashMap<Uuid, Vec<OutboundSender>>>>,
}

impl ConnectionRegistry {
    pub fn new() -> Self {
        ConnectionRegistry {
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, account_id: Uuid) -> OutboundReceiver {
        let (sender, receiver): (OutboundSender, OutboundReceiver) = mpsc::unbounded_channel();
        let mut connections = self.connections.lock().unwrap();
        connections.entry(account_id).or_insert_with(Vec::new).push(sender);
        receiver
    }

    pub fn unregister(&self, account_id: Uuid) {
        let mut connections = self.connections.lock().unwrap();
        if let Some(senders) = connections.get_mut(&account_id) {
            senders.retain(|sender| !sender.is_closed());
            if senders.is_empty() {
                connections.remove(&account_id);
            }
        }
    }

    pub fn send_to_account(&self, account_id: Uuid, message: &OutboundMessage) {
        let connections = self.connections.lock().unwrap();
        if let Some(senders) = connections.get(&account_id) {
            for sender in senders {
                let _ = sender.send(message.clone());
            }
        }
    }

    pub fn send_to_accounts(&self, account_ids: &[Uuid], message: &OutboundMessage) {
        let connections = self.connections.lock().unwrap();
        for account_id in account_ids {
            if let Some(senders) = connections.get(account_id) {
                for sender in senders {
                    let _ = sender.send(message.clone());
                }
            }
        }
    }
}
