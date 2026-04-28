use shared::schema::ws_message::{ConnectionType, WsRequest};
use std::sync::Mutex;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use crate::state::STATE;
use super::connection;

static LOBBY_TASK: WsTaskHandle = WsTaskHandle::new();
static LIVE_TASK: WsTaskHandle = WsTaskHandle::new();

struct WsTaskHandle {
    shutdown: Mutex<Option<oneshot::Sender<()>>>,
    join: Mutex<Option<JoinHandle<()>>>,
}

impl WsTaskHandle {
    const fn new() -> Self {
        WsTaskHandle {
            shutdown: Mutex::new(None),
            join: Mutex::new(None),
        }
    }

    fn start(&self, origin: &str, token: &str, connection_type: ConnectionType) {
        let (shutdown_sender, shutdown_receiver): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
        *self.shutdown.lock().unwrap() = Some(shutdown_sender);

        let join_handle: JoinHandle<()> = connection::spawn_ws(origin, token, connection_type, shutdown_receiver);
        *self.join.lock().unwrap() = Some(join_handle);
    }

    fn signal_shutdown(&self, connection_type: ConnectionType) {
        if let Some(sender) = self.shutdown.lock().unwrap().take() {
            if sender.send(()).is_err() {
                log::warn!("WS shutdown signal failed; receiver already dropped [{connection_type}]");
            }
        }
    }

    async fn await_task_end(&self) {
        if let Some(handle) = self.join.lock().unwrap().take() {
            let _ = handle.await;
        }
    }
}

fn task(connection_type: ConnectionType) -> &'static WsTaskHandle {
    match connection_type {
        ConnectionType::Lobby => &LOBBY_TASK,
        ConnectionType::Live => &LIVE_TASK,
    }
}

pub fn connect(origin: &str, token: &str, connection_type: ConnectionType) {
    task(connection_type).start(origin, token, connection_type);
}

pub fn disconnect(connection_type: ConnectionType) {
    task(connection_type).signal_shutdown(connection_type);
}

pub async fn shutdown() {
    LOBBY_TASK.signal_shutdown(ConnectionType::Lobby);
    LIVE_TASK.signal_shutdown(ConnectionType::Live);

    LOBBY_TASK.await_task_end().await;
    LIVE_TASK.await_task_end().await;
}

pub fn send(connection_type: ConnectionType, request: WsRequest) -> bool {
    let sender_guard = STATE.ws.sender(connection_type).read().unwrap();
    let Some(sender) = sender_guard.as_ref() else {
        return false;
    };

    let send_result = sender.try_send(request);
    if let Err(error) = &send_result {
        log::error!("Failed to send WsRequest [{connection_type}]; [{error}]");
    }
    send_result.is_ok()
}

pub fn is_connected(connection_type: ConnectionType) -> bool {
    STATE.ws.sender(connection_type).read().unwrap().is_some()
}
