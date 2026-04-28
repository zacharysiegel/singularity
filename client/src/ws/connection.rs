use std::ops::DerefMut;
use futures::{SinkExt, StreamExt};
use futures::stream::SplitSink;
use futures::stream::SplitStream;
use shared::schema::ws_message::{ConnectionType, WsEvent, WsRequest};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;

use crate::state::STATE;
use crate::ws::state::OUTBOUND_BUFFER_CAPACITY;
use super::route;

const BACKOFF_INITIAL: Duration = Duration::from_secs(1);
const BACKOFF_MAX: Duration = Duration::from_secs(30);

type WsSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type WsStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

enum ConnectionResult {
    Disconnected { was_connected: bool },
    Shutdown,
}

pub fn spawn_ws(
    origin: &str,
    token: &str,
    connection_type: ConnectionType,
    mut shutdown_receiver: oneshot::Receiver<()>,
) -> tokio::task::JoinHandle<()> {
    let url: String = format!("{}{}", origin, connection_type.ws_path());
    let token: String = token.to_string();

    tokio::spawn(async move {
        let mut backoff: Duration = BACKOFF_INITIAL;

        loop {
            log::info!("Connecting to WebSocket [{connection_type}] at {url}");

            let connect_result = connect_once(&url, &token, connection_type, &mut shutdown_receiver).await;

            match connect_result {
                ConnectionResult::Shutdown => {
                    log::info!("WebSocket [{connection_type}] shutdown requested");
                    break;
                }
                ConnectionResult::Disconnected { was_connected } => {
                    clear_sender(connection_type);

                    if was_connected {
                        backoff = BACKOFF_INITIAL;
                    }

                    log::warn!("WebSocket [{connection_type}] disconnected, reconnecting in {:?}", backoff);
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(BACKOFF_MAX);
                }
            }
        }

        clear_sender(connection_type);
        log::info!("WebSocket [{connection_type}] task ended");
    })
}

async fn connect_once(
    url: &str,
    token: &str,
    connection_type: ConnectionType,
    shutdown_receiver: &mut oneshot::Receiver<()>,
) -> ConnectionResult {
    let request: http::Request<()> = match build_upgrade_request(url, token) {
        Ok(request) => request,
        Err(error) => {
            log::error!("Failed to build WebSocket request [{connection_type}]; [{error}]");
            return ConnectionResult::Disconnected { was_connected: false };
        }
    };

    let connect_result = tokio_tungstenite::connect_async(request).await;
    let (ws_stream, _response) = match connect_result {
        Ok(result) => result,
        Err(error) => {
            log::error!("WebSocket connection failed [{connection_type}]; [{error}]");
            return ConnectionResult::Disconnected { was_connected: false };
        }
    };

    log::info!("WebSocket connected [{connection_type}]");

    let (mut ws_sink, mut event_stream) = ws_stream.split();
    let (request_sender, mut request_receiver): (mpsc::Sender<WsRequest>, mpsc::Receiver<WsRequest>) =
        mpsc::channel(OUTBOUND_BUFFER_CAPACITY);

    set_sender(connection_type, request_sender);

    let mut shutdown_triggered: bool = false;

    loop {
        let should_continue: bool = tokio::select! {
            event = event_stream.next() =>
                handle_inbound_event(connection_type, event),
            ws_request = request_receiver.recv() =>
                handle_outbound_request(connection_type, &mut ws_sink, ws_request).await,
            _ = &mut *shutdown_receiver => {
                shutdown_triggered = true;
                handle_shutdown(connection_type, &mut ws_sink, &mut event_stream).await
            },
        };

        if !should_continue {
            break;
        }
    }

    if shutdown_triggered {
        ConnectionResult::Shutdown
    } else {
        ConnectionResult::Disconnected { was_connected: true }
    }
}

fn build_upgrade_request(url: &str, token: &str) -> Result<http::Request<()>, http::Error> {
    let host: String = http::Uri::try_from(url)
        .map(|uri| uri.host().unwrap_or("localhost").to_string())
        .unwrap_or_else(|_| "localhost".to_string());

    http::Request::builder()
        .uri(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", tokio_tungstenite::tungstenite::handshake::client::generate_key())
        .header("Host", host)
        .body(())
}

fn handle_inbound_event(
    connection_type: ConnectionType,
    event: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>,
) -> bool {
    match event {
        Some(Ok(Message::Text(text))) => {
            let parse_result: Result<WsEvent, _> = serde_json::from_str(&text);
            match parse_result {
                Ok(ws_event) => route::route_ws_event(connection_type, ws_event),
                Err(error) => log::warn!("Failed to parse WsEvent [{connection_type}]; [{error}]"),
            }
            true
        }
        Some(Ok(Message::Binary(bytes))) => {
            let parse_result: Result<WsEvent, _> = rmp_serde::from_slice(&bytes);
            match parse_result {
                Ok(ws_event) => route::route_ws_event(connection_type, ws_event),
                Err(error) => log::warn!("Failed to parse WsEvent [{connection_type}]; [{error}]"),
            }
            true
        }
        Some(Ok(Message::Close(_))) => {
            log::info!("WebSocket server sent close [{connection_type}]");
            false
        }
        Some(Ok(Message::Ping(_))) | Some(Ok(Message::Pong(_))) | Some(Ok(_)) => true,
        Some(Err(error)) => {
            log::warn!("WebSocket receive error [{connection_type}]; [{error}]");
            false
        }
        None => {
            log::info!("WebSocket stream ended [{connection_type}]");
            false
        }
    }
}

async fn handle_outbound_request(
    connection_type: ConnectionType,
    ws_sink: &mut WsSink,
    ws_request: Option<WsRequest>,
) -> bool {
    let Some(ws_request) = ws_request else {
        return false;
    };

    let json: String = match serde_json::to_string(&ws_request) {
        Ok(json) => json,
        Err(error) => {
            log::error!("Failed to serialize WsRequest [{connection_type}]; [{error}]");
            return true;
        }
    };

    if let Err(error) = ws_sink.send(Message::Text(json.into())).await {
        log::warn!("Failed to send WsRequest [{connection_type}]; [{error}]");
        return false;
    }

    true
}

async fn handle_shutdown(
    connection_type: ConnectionType,
    ws_sink: &mut WsSink,
    event_stream: &mut WsStream,
) -> bool {
    log::info!("WebSocket shutdown signal received [{connection_type}]");

    let close_result = ws_sink.send(Message::Close(None)).await;
    if let Err(error) = close_result {
        log::warn!("WebSocket close frame failed [{connection_type}]; [{error}]");
    }

    while let Some(message) = event_stream.next().await {
        if matches!(message, Ok(Message::Close(_))) {
            break;
        }
    }

    false
}

fn set_sender(connection_type: ConnectionType, sender: mpsc::Sender<WsRequest>) {
    let mut guard = STATE.ws.sender(connection_type).write().unwrap();
    *guard = Some(sender);
}

fn clear_sender(connection_type: ConnectionType) {
    let mut guard = STATE.ws.sender(connection_type).write().unwrap();
    *guard = None;
}
