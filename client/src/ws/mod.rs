use shared::schema::ws_message::{ConnectionType, WsRequest};
use std::sync::Mutex;
use tokio::sync::oneshot;

use crate::state::STATE;

pub mod auth;
pub mod connection;
pub mod route;
pub mod state;

static LOBBY_SHUTDOWN: Mutex<Option<oneshot::Sender<()>>> = Mutex::new(None);
static LIVE_SHUTDOWN: Mutex<Option<oneshot::Sender<()>>> = Mutex::new(None);

pub fn connect_lobby(base_url: &str, token: &str) {
    let (shutdown_sender, shutdown_receiver): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
    *LOBBY_SHUTDOWN.lock().unwrap() = Some(shutdown_sender);
    connection::spawn_ws(base_url, token, ConnectionType::Lobby, shutdown_receiver);
}

pub fn connect_live(base_url: &str, token: &str) {
    let (shutdown_sender, shutdown_receiver): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
    *LIVE_SHUTDOWN.lock().unwrap() = Some(shutdown_sender);
    connection::spawn_ws(base_url, token, ConnectionType::Live, shutdown_receiver);
}

pub fn disconnect_live() {
    let sender: Option<oneshot::Sender<()>> = LIVE_SHUTDOWN.lock().unwrap().take();
    drop(sender);
}

pub fn disconnect_all() {
    let lobby_sender: Option<oneshot::Sender<()>> = LOBBY_SHUTDOWN.lock().unwrap().take();
    let live_sender: Option<oneshot::Sender<()>> = LIVE_SHUTDOWN.lock().unwrap().take();
    drop(lobby_sender);
    drop(live_sender);
}

pub fn send_lobby(request: WsRequest) -> bool {
    let sender_guard = STATE.ws.lobby_sender.read().unwrap();
    match sender_guard.as_ref() {
        Some(sender) => {
            let send_result = sender.try_send(request);
            if let Err(error) = &send_result {
                log::error!("Failed to send lobby WsRequest: {error}");
            }
            send_result.is_ok()
        }
        None => false,
    }
}

pub fn send_live(request: WsRequest) -> bool {
    let sender_guard = STATE.ws.live_sender.read().unwrap();
    match sender_guard.as_ref() {
        Some(sender) => {
            let send_result = sender.try_send(request);
            if let Err(error) = &send_result {
                log::error!("Failed to send live WsRequest: {error}");
            }
            send_result.is_ok()
        }
        None => false,
    }
}

pub fn is_lobby_connected() -> bool {
    STATE.ws.lobby_sender.read().unwrap().is_some()
}

pub fn is_live_connected() -> bool {
    STATE.ws.live_sender.read().unwrap().is_some()
}
