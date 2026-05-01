use futures::StreamExt;
use http::{header, Uri};
use shared::primitive::LoopAction;
use shared::schema::ws_message::{ConnectionType, WsRequest};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::ws::state::OUTBOUND_BUFFER_CAPACITY;
use super::{handle, state};
use super::handle::{WsSink, WsStream};

const BACKOFF_INITIAL: Duration = Duration::from_millis(100);
const BACKOFF_MAX: Duration = Duration::from_secs(5);

enum ConnectionResult {
    NotConnected { was_connected: bool },
    Shutdown,
    Impotent,
}

/// Spawns a Tokio task that maintains a WebSocket connection with automatic reconnection.
/// On disconnect, retries with exponential backoff (100ms to 5s). On shutdown signal, sends
/// a Close frame and exits.
pub fn spawn_ws(
    origin: &str,
    token: &str,
    connection_type: ConnectionType,
    shutdown_receiver: oneshot::Receiver<()>,
) -> tokio::task::JoinHandle<()> {
    let url: String = format!("{}{}", origin, connection_type.ws_path());
    let token: String = token.to_string();

    tokio::spawn(async move {
        reconnection_loop(&url, &token, connection_type, shutdown_receiver).await;
        state::clear_sender(connection_type);
        log::info!("WebSocket [{connection_type}] task ended");
    })
}

async fn reconnection_loop(
    url: &str,
    token: &str,
    connection_type: ConnectionType,
    mut shutdown_receiver: oneshot::Receiver<()>,
) {
    let mut backoff: Duration = BACKOFF_INITIAL;

    loop {
        log::info!("Connecting to WebSocket [{connection_type}] [{url}]");

        let connect_result: ConnectionResult = connect_once(url, token, connection_type, &mut shutdown_receiver).await;
        match connect_result {
            ConnectionResult::Shutdown => {
                log::info!("WebSocket [{connection_type}] shutdown requested");
                break;
            }
            ConnectionResult::NotConnected { was_connected } => {
                state::clear_sender(connection_type);

                if was_connected {
                    backoff = BACKOFF_INITIAL;
                }

                log::warn!("WebSocket [{connection_type}] disconnected, reconnecting in {:?}", backoff);
                tokio::time::sleep(backoff).await;
                backoff = (backoff * 2).min(BACKOFF_MAX);
            }
            ConnectionResult::Impotent => break,
        }
    }
}

async fn connect_once(
    url: &str,
    token: &str,
    connection_type: ConnectionType,
    shutdown_receiver: &mut oneshot::Receiver<()>,
) -> ConnectionResult {
    let request: http::Request<()> = match upgrade_request(url, token) {
        Ok(request) => request,
        Err(error) => {
            log::error!("Failed to build WebSocket request; [{connection_type}] [{error}]");
            return ConnectionResult::Impotent;
        }
    };

    let ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>> = match tokio_tungstenite::connect_async(request).await {
        Ok((stream, _response)) => stream,
        Err(error) => {
            log::error!("WebSocket connection failed; [{connection_type}] [{error}]");
            return ConnectionResult::NotConnected { was_connected: false };
        }
    };

    log::info!("WebSocket connected [{connection_type}]");

    let hook_token: String = token.to_string();
    tokio::spawn(async move {
        super::hook::after_connect(connection_type, &hook_token).await;
    });

    let (mut request_sink, mut event_stream): (WsSink, WsStream) = ws_stream.split();
    let (request_sender, mut request_receiver): (mpsc::Sender<WsRequest>, mpsc::Receiver<WsRequest>) =
        mpsc::channel(OUTBOUND_BUFFER_CAPACITY);
    state::set_sender(connection_type, request_sender);

    let mut shutdown_triggered: bool = false;
    loop {
        let loop_action: LoopAction = tokio::select! {
            event = event_stream.next() =>
                handle::handle_inbound_event(connection_type, event),
            ws_request = request_receiver.recv() =>
                handle::handle_outbound_request(connection_type, &mut request_sink, ws_request).await,
            _ = &mut *shutdown_receiver => {
                shutdown_triggered = true;
                handle::handle_shutdown(connection_type, &mut request_sink, &mut event_stream).await
            },
        };

        if loop_action == LoopAction::Stop {
            break;
        }
    }

    if shutdown_triggered {
        ConnectionResult::Shutdown
    } else {
        ConnectionResult::NotConnected { was_connected: true }
    }
}

fn upgrade_request(url: &str, token: &str) -> Result<http::Request<()>, http::Error> {
    let uri: Uri = Uri::try_from(url).expect("invalid WebSocket URL");
    let host: &str = uri.host().expect("WebSocket URL missing host");

    http::Request::builder()
        .uri(url)
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .header(header::CONNECTION, "Upgrade")
        .header(header::UPGRADE, "websocket")
        .header(header::SEC_WEBSOCKET_VERSION, "13")
        .header(header::SEC_WEBSOCKET_KEY, tokio_tungstenite::tungstenite::handshake::client::generate_key())
        .header(header::HOST, host)
        .body(())
}
