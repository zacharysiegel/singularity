use crate::lobby_error::{LobbyError, ResultExtLobbyError};
use crate::session::session_extractor::AuthenticatedAccount;
use crate::ws::connection_type::ConnectionType;
use actix_web::{rt, web, HttpRequest, HttpResponse};
use actix_ws::{CloseCode, CloseReason, Message, MessageStream, ProtocolError, Session};
use bytes::Bytes;
use bytestring::ByteString;
use std::time::{Duration, Instant};
use uuid::Uuid;

pub async fn ws_handler(
    request: HttpRequest,
    body: web::Payload,
    auth: AuthenticatedAccount,
    connection_type: ConnectionType,
    rate_limit_interval: Option<Duration>,
) -> Result<HttpResponse, LobbyError> {
    let account_id: Uuid = auth.account_id;

    let (upgrade_response, mut ws_session, mut message_stream): (HttpResponse, Session, MessageStream) =
        actix_ws::handle(&request, body).or_bad_request()?;

    rt::spawn(async move {
        let mut last_delivery: Instant = Instant::now(); // Ignored if rate limiting is disabled

        while let Some(message_result) = message_stream.recv().await {
            let message: Message = match message_result {
                Ok(message) => message,
                Err(error) => {
                    handle_receive_error(ws_session.clone(), connection_type, account_id, error).await;
                    break;
                }
            };

            let should_continue: bool = match message {
                Message::Text(text) => {
                    throttle(rate_limit_interval, &mut last_delivery).await;
                    handle_text(&mut ws_session, connection_type, account_id, text).await
                }
                Message::Binary(bytes) => {
                    throttle(rate_limit_interval, &mut last_delivery).await;
                    handle_binary(&mut ws_session, connection_type, account_id, bytes).await
                }
                Message::Ping(payload) =>
                    handle_ping(&mut ws_session, connection_type, account_id, &payload).await,
                Message::Close(reason) =>
                    handle_close(ws_session.clone(), connection_type, account_id, reason).await,
                _ => true,
            };

            if !should_continue {
                break;
            }
        }

        log::info!("WebSocket task ended [{connection_type}] [{account_id}]");
    });

    log::info!("WebSocket connection opened [{connection_type}] [{account_id}]");
    Ok(upgrade_response)
}

/// The "live" WebSocket connection is used by the client while in-game. To avoid frame rate spikes
/// or denial of service, we enforce a rate limit on outbound messages through this socket.
async fn throttle(
    rate_limit_interval: Option<Duration>,
    last_delivery: &mut Instant
) {
    let Some(rate_limit_interval) = rate_limit_interval else {
        return;
    };

    let elapsed: Duration = last_delivery.elapsed();
    if elapsed < rate_limit_interval {
        let sleep_duration: Duration = rate_limit_interval - elapsed;
        tokio::time::sleep(sleep_duration).await;
    }
    *last_delivery = Instant::now();
}

async fn handle_text(
    ws_session: &mut Session,
    connection_type: ConnectionType,
    account_id: Uuid,
    text: ByteString,
) -> bool {
    log::trace!(
        "WebSocket TEXT [{connection_type}] [{account_id}]: {} bytes",
        text.as_bytes().len(),
    );
    // TODO: parse and route inbound messages
    let echo: String = format!("echo: {}", text);
    ws_session.text(echo).await.is_ok()
}

async fn handle_binary(
    ws_session: &mut Session,
    connection_type: ConnectionType,
    account_id: Uuid,
    bytes: Bytes,
) -> bool {
    log::trace!(
        "WebSocket BINARY [{connection_type}] [{account_id}]: {} bytes",
        bytes.len(),
    );
    // TODO: parse MessagePack inbound messages
    ws_session.binary(bytes).await.is_ok()
}

async fn handle_receive_error(
    ws_session: Session,
    connection_type: ConnectionType,
    account_id: Uuid,
    error: ProtocolError,
) {
    log::warn!("WebSocket receive error [{connection_type}] [{account_id}]: {error}");
    let close_reason: CloseReason = CloseReason {
        code: CloseCode::Protocol,
        description: Some(error.to_string()),
    };
    let _ = ws_session.close(Some(close_reason)).await;
}

async fn handle_ping(
    ws_session: &mut Session,
    connection_type: ConnectionType,
    account_id: Uuid,
    payload: &[u8],
) -> bool {
    log::trace!("WebSocket PING [{connection_type}] [{account_id}]");
    ws_session.pong(payload).await.is_ok()
}

async fn handle_close(
    ws_session: Session,
    connection_type: ConnectionType,
    account_id: Uuid,
    reason: Option<CloseReason>,
) -> bool {
    log::debug!("WebSocket CLOSE [{connection_type}] [{account_id}]");
    let _ = ws_session.close(reason).await;
    false
}
