use dashmap::DashMap;
use shared::schema::ws_message::OutboundMessage;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

pub type OutboundSender = mpsc::UnboundedSender<OutboundMessage>;
pub type OutboundReceiver = mpsc::UnboundedReceiver<OutboundMessage>;

/// Registry of active WebSocket connections, mapping account_id to outbound message senders.
/// Uses DashMap for per-shard locking — reads (broadcasts) and writes (register/unregister)
/// to different accounts don't contend.
#[derive(Debug, Clone)]
pub struct ConnectionRegistry {
    connections: Arc<DashMap<Uuid, Vec<OutboundSender>>>,
}

impl ConnectionRegistry {
    pub fn new() -> Self {
        ConnectionRegistry {
            connections: Arc::new(DashMap::new()),
        }
    }

    pub fn register(&self, account_id: Uuid) -> OutboundReceiver {
        let (sender, receiver): (OutboundSender, OutboundReceiver) = mpsc::unbounded_channel();
        self.connections.entry(account_id).or_insert_with(Vec::new).push(sender);
        receiver
    }

    pub fn unregister(&self, account_id: Uuid) {
        let mut should_remove: bool = false;
        if let Some(mut senders) = self.connections.get_mut(&account_id) {
            senders.retain(|sender| !sender.is_closed());
            should_remove = senders.is_empty();
        }
        if should_remove {
            self.connections.remove(&account_id);
        }
    }

    pub fn send_to_account(&self, account_id: Uuid, message: &OutboundMessage) {
        if let Some(senders) = self.connections.get(&account_id) {
            for sender in senders.iter() {
                let _ = sender.send(message.clone());
            }
        }
    }

    pub fn send_to_accounts(&self, account_ids: &[Uuid], message: &OutboundMessage) {
        for account_id in account_ids {
            if let Some(senders) = self.connections.get(account_id) {
                for sender in senders.iter() {
                    let _ = sender.send(message.clone());
                }
            }
        }
    }
}
