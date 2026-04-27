use shared::schema::ws_message::{ConnectionType, WsRequest};
use std::sync::RwLock;
use tokio::sync::mpsc::Sender;
use crate::state::STATE;

pub const OUTBOUND_BUFFER_CAPACITY: usize = 512;

#[derive(Debug)]
pub struct WsState {
    pub lobby_sender: RwLock<Option<Sender<WsRequest>>>,
    pub live_sender: RwLock<Option<Sender<WsRequest>>>,
    pub last_error: RwLock<Option<String>>,
    pub token: RwLock<Option<String>>,
}

impl Default for WsState {
    fn default() -> Self {
        WsState {
            lobby_sender: RwLock::new(None),
            live_sender: RwLock::new(None),
            last_error: RwLock::new(None),
            token: RwLock::new(None),
        }
    }
}

impl WsState {
    pub fn sender(
        &self,
        connection_type: ConnectionType,
    ) -> &RwLock<Option<Sender<WsRequest>>> {
        match connection_type {
            ConnectionType::Lobby => &self.lobby_sender,
            ConnectionType::Live => &self.live_sender,
        }
    }

    pub fn is_lobby_connected(&self) -> bool {
        self.lobby_sender.read().unwrap().is_some()
    }

    pub fn is_live_connected(&self) -> bool {
        self.live_sender.read().unwrap().is_some()
    }
}
