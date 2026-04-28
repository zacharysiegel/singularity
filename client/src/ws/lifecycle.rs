use shared::schema::ws_message::{ConnectionType, WsRequest};
use std::sync::Mutex;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use super::connection;
use crate::state::STATE;

static LOBBY_TASK: WsTaskHandle = WsTaskHandle::new(ConnectionType::Lobby);
static LIVE_TASK: WsTaskHandle = WsTaskHandle::new(ConnectionType::Live);
static ALL_TASKS: &[&WsTaskHandle] = &[&LOBBY_TASK, &LIVE_TASK];

struct WsTaskHandle {
    connection_type: ConnectionType,
    shutdown: Mutex<Option<oneshot::Sender<()>>>,
    join: Mutex<Option<JoinHandle<()>>>,
}

impl WsTaskHandle {
    const fn new(connection_type: ConnectionType) -> Self {
        WsTaskHandle {
            connection_type,
            shutdown: Mutex::new(None),
            join: Mutex::new(None),
        }
    }

    fn start(&self, origin: &str, token: &str) {
        let (shutdown_sender, shutdown_receiver): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel();
        *self.shutdown.lock().unwrap() = Some(shutdown_sender);

        let join_handle: JoinHandle<()> = connection::spawn_ws(origin, token, self.connection_type, shutdown_receiver);
        *self.join.lock().unwrap() = Some(join_handle);
    }

    fn signal_shutdown(&self) {
        let sender: Option<oneshot::Sender<()>> = self.shutdown.lock().unwrap().take();
        match sender {
            Some(sender) => {
                if sender.send(()).is_err() {
                    log::warn!("WS shutdown signal failed; receiver already dropped [{}]", self.connection_type);
                }
            }
            None => {
                log::debug!("WS shutdown signal skipped; not connected [{}]", self.connection_type);
            }
        }
    }

    async fn await_task_end(&self) {
        let handle: Option<JoinHandle<()>> = self.join.lock().unwrap().take();
        match handle {
            Some(handle) => {
                let _ = handle.await;
            }
            None => {
                log::debug!("WS await_task_end skipped; no handle [{}]", self.connection_type);
            }
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
    task(connection_type).start(origin, token);
}

pub fn disconnect(connection_type: ConnectionType) {
    task(connection_type).signal_shutdown();
}

pub async fn shutdown() {
    for task in ALL_TASKS {
        task.signal_shutdown();
        task.await_task_end().await;
    }
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
