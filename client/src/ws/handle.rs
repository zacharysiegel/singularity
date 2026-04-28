use futures::SinkExt;
use futures::StreamExt;
use futures::stream::SplitSink;
use futures::stream::SplitStream;
use shared::environment::RuntimeEnvironment;
use shared::primitive::LoopAction;
use shared::schema::ws_message::{ConnectionType, WsEvent, WsRequest};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use super::route;
use std::time::Duration;

const CLOSE_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(1);

pub type WsSink = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
pub type WsStream = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

pub fn handle_inbound_event(
    connection_type: ConnectionType,
    event: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>,
) -> LoopAction {
    let Some(event) = event else {
        log::info!("WebSocket stream ended [{connection_type}]");
        return LoopAction::Stop;
    };

    let event: Message = match event {
        Ok(event) => event,
        Err(error) => {
            log::warn!("WebSocket receive error; [{connection_type}] [{error}]");
            return LoopAction::Stop;
        }
    };

    match event {
        Message::Text(text) => {
            let parse_result: Result<WsEvent, _> = serde_json::from_str(&text);
            match parse_result {
                Ok(ws_event) => route::route_ws_event(connection_type, ws_event),
                Err(error) => log::warn!(
                    "Failed to parse WsEvent; [{connection_type}] [{error}] [{}b]",
                    text.len()
                ),
            }
            LoopAction::Continue
        }
        Message::Binary(bytes) => {
            let parse_result: Result<WsEvent, _> = rmp_serde::from_slice(&bytes);
            match parse_result {
                Ok(ws_event) => route::route_ws_event(connection_type, ws_event),
                Err(error) => log::warn!(
                    "Failed to parse WsEvent; [{connection_type}] [{error}] [{}b]",
                    bytes.len()
                ),
            }
            LoopAction::Continue
        }
        Message::Close(_) => {
            log::info!("WebSocket server sent close [{connection_type}]");
            LoopAction::Stop
        }
        Message::Ping(_) | Message::Pong(_) | Message::Frame(_) => LoopAction::Continue,
    }
}

pub async fn handle_outbound_request(
    connection_type: ConnectionType,
    ws_sink: &mut WsSink,
    ws_request: Option<WsRequest>,
) -> LoopAction {
    let Some(ws_request) = ws_request else {
        return LoopAction::Stop;
    };

    let message: Message = match RuntimeEnvironment::default().is_debug() {
        true => match json_message(ws_request) {
            Ok(message) => message,
            Err(loop_action) => return loop_action,
        },
        false => match msgpack_message(ws_request) {
            Ok(message) => message,
            Err(loop_action) => return loop_action,
        },
    };

    if let Err(error) = ws_sink.send(message).await {
        log::warn!("Failed to send WsRequest; [{connection_type}] [{error}]");
        return LoopAction::Stop;
    }

    LoopAction::Continue
}

fn json_message(request: WsRequest) -> Result<Message, LoopAction> {
    let json: String = match serde_json::to_string(&request) {
        Ok(json) => json,
        Err(error) => {
            log::error!("Failed to serialize WsRequest as JSON; [{error}]");
            return Err(LoopAction::Continue);
        }
    };
    Ok(Message::Text(json.into()))
}

fn msgpack_message(request: WsRequest) -> Result<Message, LoopAction> {
    let bytes: Vec<u8> = match rmp_serde::to_vec_named(&request) {
        Ok(bytes) => bytes,
        Err(error) => {
            log::error!("Failed to serialize WsRequest as MessagePack; [{error}]");
            return Err(LoopAction::Continue);
        }
    };
    Ok(Message::Binary(bytes.into()))
}

pub async fn handle_shutdown(
    connection_type: ConnectionType,
    ws_sink: &mut WsSink,
    event_stream: &mut WsStream,
) -> LoopAction {
    log::info!("WebSocket shutdown signal received [{connection_type}]");

    let close_result: Result<(), tokio_tungstenite::tungstenite::Error> = ws_sink.send(Message::Close(None)).await;
    if let Err(error) = close_result {
        log::warn!("WebSocket close frame failed; [{connection_type}] [{error}]");
    }

    // Wait for the server's Close response before dropping the stream
    let close_wait = async {
        while let Some(message) = event_stream.next().await {
            if matches!(message, Ok(Message::Close(_))) {
                break;
            }
        }
    };
    if tokio::time::timeout(CLOSE_HANDSHAKE_TIMEOUT, close_wait).await.is_err() {
        log::warn!("WebSocket close handshake timed out; [{connection_type}]");
    }

    LoopAction::Stop
}
