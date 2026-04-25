use dashmap::DashMap;
use shared::schema::ws_message::WsEvent;
use std::sync::LazyLock;
use dashmap::mapref::one::RefMut;
use tokio::sync::mpsc;
use uuid::Uuid;

use super::connection_type::ConnectionType;

pub type WsSender = mpsc::Sender<WsEvent>;
pub type WsReceiver = mpsc::Receiver<WsEvent>;

/// Unit: messages
const OUTBOUND_BUFFER_CAPACITY: usize = 512;

struct AccountConnections {
    lobby: Option<WsSender>,
    live: Option<WsSender>,
}

impl AccountConnections {
    fn empty() -> Self {
        AccountConnections {
            lobby: None,
            live: None,
        }
    }

    fn sender(&self, connection_type: ConnectionType) -> &Option<WsSender> {
        match connection_type {
            ConnectionType::Lobby => &self.lobby,
            ConnectionType::Live => &self.live,
        }
    }

    fn sender_mut(&mut self, connection_type: ConnectionType) -> &mut Option<WsSender> {
        match connection_type {
            ConnectionType::Lobby => &mut self.lobby,
            ConnectionType::Live => &mut self.live,
        }
    }

    fn is_empty(&self) -> bool {
        self.lobby.is_none() && self.live.is_none()
    }
}

/// Global registry of active WebSocket connections, mapping account_id to connection senders.
/// Uses DashMap for per-shard locking so that operations on different accounts don't contend.
static CONNECTIONS: LazyLock<DashMap<Uuid, AccountConnections>> = LazyLock::new(DashMap::new);

pub fn register(account_id: Uuid, connection_type: ConnectionType) -> WsReceiver {
    let (sender, receiver): (WsSender, WsReceiver) = mpsc::channel(OUTBOUND_BUFFER_CAPACITY);
    let mut entry: RefMut<Uuid, AccountConnections> = CONNECTIONS.entry(account_id).or_insert_with(AccountConnections::empty);
    *entry.sender_mut(connection_type) = Some(sender);
    receiver
}

pub fn unregister(account_id: Uuid, connection_type: ConnectionType) {
    let mut should_remove: bool = false;
    if let Some(mut connections) = CONNECTIONS.get_mut(&account_id) {
        let sender: &mut Option<WsSender> = connections.sender_mut(connection_type);
        *sender = None;
        should_remove = connections.is_empty();
    }
    if should_remove {
        CONNECTIONS.remove(&account_id);
    }
}

pub fn send_to_account(account_id: Uuid, connection_type: ConnectionType, message: &WsEvent) {
    let Some(connections) = CONNECTIONS.get(&account_id) else { return };
    let Some(sender) = connections.sender(connection_type) else { return };
    let send_result: Result<(), mpsc::error::TrySendError<WsEvent>> = sender.try_send(message.clone());
    if let Err(error) = send_result {
        log::warn!("Failed to send to [{connection_type}] [{account_id}]: {error}");
    }
}

pub fn send_to_accounts(account_ids: &[Uuid], connection_type: ConnectionType, message: &WsEvent) {
    for account_id in account_ids {
        send_to_account(*account_id, connection_type, message);
    }
}
