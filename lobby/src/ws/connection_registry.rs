use dashmap::DashMap;
use shared::schema::ws_message::WsEvent;
use std::sync::LazyLock;
use dashmap::mapref::entry::Entry;
use dashmap::mapref::one::RefMut;
use tokio::sync::mpsc;
use uuid::Uuid;

use super::connection_type::ConnectionType;

pub type WsSender = mpsc::Sender<WsEvent>;
pub type WsReceiver = mpsc::Receiver<WsEvent>;

/// Unit: messages
const OUTBOUND_BUFFER_CAPACITY: usize = 512;

/// Global registry of active WebSocket connections, mapping account_id to per-session connection senders/receivers.
/// Uses DashMap for per-shard locking so that operations on different accounts don't contend.
/// Sessions are stored in a Vec (linear scan) rather than a HashMap because the number of concurrent sessions per account is realistically 1-3.
static CONNECTIONS: LazyLock<DashMap<Uuid, Vec<SessionConnections>>> = LazyLock::new(DashMap::new);

struct SessionConnections {
    session_id: Uuid,
    lobby: Option<WsSender>,
    live: Option<WsSender>,
}

impl SessionConnections {
    fn empty(session_id: Uuid) -> Self {
        SessionConnections {
            session_id,
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

pub fn register(account_id: Uuid, session_id: Uuid, connection_type: ConnectionType) -> WsReceiver {
    let (sender, receiver): (WsSender, WsReceiver) = mpsc::channel(OUTBOUND_BUFFER_CAPACITY);
    let mut sessions: RefMut<Uuid, Vec<SessionConnections>> = CONNECTIONS.entry(account_id).or_insert_with(Vec::new);

    let existing_session: Option<&mut SessionConnections> = sessions.iter_mut()
        .find(|session| session.session_id == session_id);

    let session_connections: &mut SessionConnections = match existing_session {
        Some(existing) => existing,
        None => {
            sessions.push(SessionConnections::empty(session_id));
            sessions.last_mut().expect("session was just pushed into the vector")
        }
    };
    *session_connections.sender_mut(connection_type) = Some(sender);

    receiver
}

pub fn unregister(account_id: Uuid, session_id: Uuid, connection_type: ConnectionType) {
    // We need to use the "entry" API here in order to use the same write lock for "get_mut" and "remove"
    let Entry::Occupied(mut occupied) = CONNECTIONS.entry(account_id) else {
        return
    };

    let session_connections: Option<&mut SessionConnections> = occupied.get_mut()
        .iter_mut()
        .find(|session| session.session_id == session_id);
    if let Some(session_connections) = session_connections {
        *session_connections.sender_mut(connection_type) = None;
    }

    occupied.get_mut()
        .retain(|session| !session.is_empty());

    if occupied.get().is_empty() {
        occupied.remove();
    }
}

pub fn send_to_account(account_id: Uuid, connection_type: ConnectionType, message: &WsEvent) {
    let Some(sessions) = CONNECTIONS.get(&account_id) else { return };
    for session_connections in sessions.iter() {
        let Some(sender) = session_connections.sender(connection_type) else { continue };
        let send_result: Result<(), mpsc::error::TrySendError<WsEvent>> = sender.try_send(message.clone());
        if let Err(error) = send_result {
            log::warn!(
                "Failed to send to [{connection_type}] [{account_id}] [{}]: {error}",
                session_connections.session_id,
            );
        }
    }
}

pub fn send_to_accounts(account_ids: &[Uuid], connection_type: ConnectionType, message: &WsEvent) {
    for account_id in account_ids {
        send_to_account(*account_id, connection_type, message);
    }
}
