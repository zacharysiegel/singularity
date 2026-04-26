use futures::{SinkExt, StreamExt};
use shared::schema::ws_message::{ConnectionType, WsEvent, WsRequest};
use std::time::Duration;
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::tungstenite::Message;

use crate::state::STATE;
use crate::ws::state::OUTBOUND_BUFFER_CAPACITY;
use super::route;

const BACKOFF_INITIAL: Duration = Duration::from_secs(1);
const BACKOFF_MAX: Duration = Duration::from_secs(30);

pub fn spawn_ws(
    base_url: &str,
    token: &str,
    connection_type: ConnectionType,
    mut shutdown_receiver: oneshot::Receiver<()>,
) -> tokio::task::JoinHandle<()> {
    let url: String = format!("{}{}", base_url, connection_type.ws_path());
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

enum ConnectionResult {
    Disconnected { was_connected: bool },
    Shutdown,
}

async fn connect_once(
    url: &str,
    token: &str,
    connection_type: ConnectionType,
    shutdown_receiver: &mut oneshot::Receiver<()>,
) -> ConnectionResult {
    let request: http::Request<()> = match http::Request::builder()
        .uri(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header("Sec-WebSocket-Key", tokio_tungstenite::tungstenite::handshake::client::generate_key())
        .header("Host", http::Uri::try_from(url).map(|uri| uri.host().unwrap_or("localhost").to_string()).unwrap_or_else(|_| "localhost".to_string()))
        .body(())
    {
        Ok(request) => request,
        Err(error) => {
            log::error!("Failed to build WebSocket request [{connection_type}]: {error}");
            return ConnectionResult::Disconnected { was_connected: false };
        }
    };

    let connect_result = tokio_tungstenite::connect_async(request).await;
    let (ws_stream, _response) = match connect_result {
        Ok(result) => result,
        Err(error) => {
            log::error!("WebSocket connection failed [{connection_type}]: {error}");
            return ConnectionResult::Disconnected { was_connected: false };
        }
    };

    log::info!("WebSocket connected [{connection_type}]");

    let (mut ws_sink, mut event_stream) = ws_stream.split();
    let (request_sender, mut request_receiver): (mpsc::Sender<WsRequest>, mpsc::Receiver<WsRequest>) =
        mpsc::channel(OUTBOUND_BUFFER_CAPACITY);

    set_sender(connection_type, request_sender);

    loop {
        tokio::select! {
            event = event_stream.next() => {
                match event {
                    Some(Ok(Message::Text(text))) => {
                        let parse_result: Result<WsEvent, _> = serde_json::from_str(&text);
                        match parse_result {
                            Ok(ws_event) => route::route_ws_event(connection_type, ws_event),
                            Err(error) => log::warn!("Failed to parse WsEvent [{connection_type}]: {error}"),
                        }
                    }
                    Some(Ok(Message::Binary(bytes))) => {
                        let parse_result: Result<WsEvent, _> = rmp_serde::from_slice(&bytes);
                        match parse_result {
                            Ok(ws_event) => route::route_ws_event(connection_type, ws_event),
                            Err(error) => log::warn!("Failed to parse WsEvent [{connection_type}]: {error}"),
                        }
                    }
                    Some(Ok(Message::Ping(_))) => {}
                    Some(Ok(Message::Pong(_))) => {}
                    Some(Ok(Message::Close(_))) => {
                        log::info!("WebSocket server sent close [{connection_type}]");
                        return ConnectionResult::Disconnected { was_connected: true };
                    }
                    Some(Ok(_)) => {}
                    Some(Err(error)) => {
                        log::warn!("WebSocket receive error [{connection_type}]: {error}");
                        return ConnectionResult::Disconnected { was_connected: true };
                    }
                    None => {
                        log::info!("WebSocket stream ended [{connection_type}]");
                        return ConnectionResult::Disconnected { was_connected: true };
                    }
                }
            }
            ws_request = request_receiver.recv() => {
                match ws_request {
                    Some(ws_request) => {
                        let json: String = match serde_json::to_string(&ws_request) {
                            Ok(json) => json,
                            Err(error) => {
                                log::error!("Failed to serialize WsRequest [{connection_type}]: {error}");
                                continue;
                            }
                        };
                        if let Err(error) = ws_sink.send(Message::Text(json.into())).await {
                            log::warn!("Failed to send WsRequest [{connection_type}]: {error}");
                            return ConnectionResult::Disconnected { was_connected: true };
                        }
                    }
                    None => {
                        return ConnectionResult::Disconnected { was_connected: true };
                    }
                }
            }
            _ = &mut *shutdown_receiver => {
                log::info!("WebSocket shutdown signal received [{connection_type}]");
                let _ = ws_sink.close().await;
                return ConnectionResult::Shutdown;
            }
        }
    }
}

fn set_sender(connection_type: ConnectionType, sender: mpsc::Sender<WsRequest>) {
    let mut guard = STATE.ws.sender(connection_type).write().unwrap();
    *guard = Some(sender);
}

fn clear_sender(connection_type: ConnectionType) {
    let mut guard = STATE.ws.sender(connection_type).write().unwrap();
    *guard = None;
}
