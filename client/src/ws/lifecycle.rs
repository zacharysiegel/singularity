use shared::schema::ws_message::{ConnectionType, WsRequest};
use std::sync::Mutex;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use crate::state::STATE;
use super::connection;

static LOBBY_SHUTDOWN: Mutex<Option<oneshot::Sender<()>>> = Mutex::new(None);
static LIVE_SHUTDOWN: Mutex<Option<oneshot::Sender<()>>> = Mutex::new(None);
static LOBBY_HANDLE: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
static LIVE_HANDLE: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);

pub fn connect_lobby(origin: &str, token: &str) {
    let (shutdown_sender, shutdown_receiver): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
    *LOBBY_SHUTDOWN.lock().unwrap() = Some(shutdown_sender);
    let join_handle: JoinHandle<()> = connection::spawn_ws(origin, token, ConnectionType::Lobby, shutdown_receiver);
    *LOBBY_HANDLE.lock().unwrap() = Some(join_handle);
}

pub fn connect_live(origin: &str, token: &str) {
    let (shutdown_sender, shutdown_receiver): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
    *LIVE_SHUTDOWN.lock().unwrap() = Some(shutdown_sender);
    let join_handle: JoinHandle<()> = connection::spawn_ws(origin, token, ConnectionType::Live, shutdown_receiver);
    *LIVE_HANDLE.lock().unwrap() = Some(join_handle);
}

pub fn disconnect_live() {
    drop(LIVE_SHUTDOWN.lock().unwrap().take());
}

pub async fn shutdown() {
    drop(LOBBY_SHUTDOWN.lock().unwrap().take());
    drop(LIVE_SHUTDOWN.lock().unwrap().take());

    if let Some(handle) = LOBBY_HANDLE.lock().unwrap().take() {
        let _ = handle.await;
    }
    if let Some(handle) = LIVE_HANDLE.lock().unwrap().take() {
        let _ = handle.await;
    }
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
