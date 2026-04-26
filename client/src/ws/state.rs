use shared::schema::ws_message::WsRequest;
use std::sync::RwLock;
use tokio::sync::mpsc::Sender;

pub const OUTBOUND_BUFFER_CAPACITY: usize = 512;

#[derive(Debug)]
pub struct WsState {
    pub lobby_sender: RwLock<Option<Sender<WsRequest>>>,
    pub live_sender: RwLock<Option<Sender<WsRequest>>>,
    pub last_error: RwLock<Option<String>>,
    pub token: RwLock<Option<String>>,
}

impl WsState {
    pub fn new() -> Self {
        WsState {
            lobby_sender: RwLock::new(None),
            live_sender: RwLock::new(None),
            last_error: RwLock::new(None),
            token: RwLock::new(None),
        }
    }

    pub fn sender(&self, connection_type: shared::schema::ws_message::ConnectionType) -> &RwLock<Option<Sender<WsRequest>>> {
        match connection_type {
            shared::schema::ws_message::ConnectionType::Lobby => &self.lobby_sender,
            shared::schema::ws_message::ConnectionType::Live => &self.live_sender,
        }
    }
}
