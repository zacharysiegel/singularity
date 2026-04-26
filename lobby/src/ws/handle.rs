use crate::lobby_error::{LobbyError, ResultExtLobbyError};
use crate::session::session_extractor::AuthenticatedAccount;
use crate::ws::connection_registry;
use crate::ws::connection_type::ConnectionType;
use crate::ws::router;
use actix_web::{rt, web, HttpRequest, HttpResponse};
use actix_ws::{CloseCode, CloseReason, Message, MessageStream, ProtocolError, Session};
use shared::schema::ws_message::{WsRequest, WsEvent};
use sqlx::PgPool;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

pub async fn ws_handler(
    request: HttpRequest,
    body: web::Payload,
    auth: AuthenticatedAccount,
    pg_pool: web::Data<PgPool>,
    connection_type: ConnectionType,
    rate_limit_interval: Option<Duration>,
) -> Result<HttpResponse, LobbyError> {
    let account_id: Uuid = auth.account_id;
    let session_id: Uuid = auth.session_id;

    let (upgrade_response, mut ws_session, mut request_stream): (HttpResponse, Session, MessageStream) =
        actix_ws::handle(&request, body).or_bad_request()?;

    let mut event_receiver: Receiver<Arc<WsEvent>> = connection_registry::register(account_id, session_id, connection_type);
    let pg_pool: PgPool = pg_pool.get_ref().clone();

    rt::spawn(async move {
        let mut last_outbound_delivery: Instant = Instant::now();

        loop {
            let should_continue: bool = tokio::select! {
                request = request_stream.recv() =>
                    handle_inbound_frame(&pg_pool, connection_type, account_id, request, &mut ws_session).await,
                event = event_receiver.recv() =>
                    handle_outbound_event(rate_limit_interval, &mut last_outbound_delivery, event, &mut ws_session).await,
            };

            if !should_continue {
                break;
            }
        }

        connection_registry::unregister(account_id, session_id, connection_type);
        log::info!("WebSocket connection closed [{connection_type}] [{account_id}]");
    });

    log::info!("WebSocket connection opened [{connection_type}] [{account_id}]");
    Ok(upgrade_response)
}

/// The "live" WebSocket connection is used by the client while in-game. To avoid frame rate spikes
/// or denial of service, we enforce a rate limit on outbound messages through this socket.
async fn throttle(
    rate_limit_interval: Option<Duration>,
    last_delivery: &mut Instant,
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

async fn handle_inbound_frame(
    pool: &PgPool,
    connection_type: ConnectionType,
    account_id: Uuid,
    ws_message: Option<Result<Message, ProtocolError>>,
    ws_session: &mut Session,
) -> bool {
    match ws_message {
        Some(Ok(Message::Text(text))) => {
            handle_text_message(pool, connection_type, account_id, &text, ws_session).await;
            true
        }
        Some(Ok(Message::Binary(bytes))) => {
            handle_binary_message(pool, connection_type, account_id, &bytes, ws_session).await;
            true
        }
        Some(Ok(Message::Ping(payload))) =>
            handle_ping(ws_session, connection_type, account_id, &payload).await,
        Some(Ok(Message::Close(reason))) =>
            handle_close(ws_session.clone(), connection_type, account_id, reason).await,
        Some(Ok(_)) => true,
        Some(Err(error)) => {
            handle_receive_error(ws_session.clone(), connection_type, account_id, error).await;
            false
        }
        None => false,
    }
}

async fn handle_outbound_event(
    rate_limit_interval: Option<Duration>,
    last_outbound_delivery: &mut Instant,
    outbound: Option<Arc<WsEvent>>,
    ws_session: &mut Session,
) -> bool {
    let Some(ws_event) = outbound else {
        return false;
    };

    throttle(rate_limit_interval, last_outbound_delivery).await;
    send_outbound_json(ws_session, &ws_event).await.is_ok()
}

/// We assume a text message is in the JSON format. We respond in kind.
async fn handle_text_message(
    pool: &PgPool,
    connection_type: ConnectionType,
    account_id: Uuid,
    text: &str,
    ws_session: &mut Session,
) {
    log::trace!(
        "WebSocket TEXT [{connection_type}] [{account_id}]: {} bytes",
        text.len(),
    );

    let parse_result: Result<WsRequest, _> = serde_json::from_str(text);
    match parse_result {
        Ok(ws_request) => {
            if let Err(error) = router::route_ws_request(pool, connection_type, account_id, ws_request).await {
                let _ = send_outbound_json(ws_session, &WsEvent::Error { message: error.message.clone() }).await;
            }
        }
        Err(error) => {
            let _ = send_outbound_json(ws_session, &WsEvent::Error { message: format!("invalid message: {error}") }).await;
        }
    }
}

/// We assume a binary message is in the MessagePack format. We respond in kind.
async fn handle_binary_message(
    pool: &PgPool,
    connection_type: ConnectionType,
    account_id: Uuid,
    bytes: &[u8],
    ws_session: &mut Session,
) {
    log::trace!(
        "WebSocket BINARY [{connection_type}] [{account_id}]: {} bytes",
        bytes.len(),
    );

    let parse_result: Result<WsRequest, _> = rmp_serde::from_slice(bytes);
    match parse_result {
        Ok(ws_request) => {
            if let Err(error) = router::route_ws_request(pool, connection_type, account_id, ws_request).await {
                let _ = send_outbound_msgpack(ws_session, &WsEvent::Error { message: error.message.clone() }).await;
            }
        }
        Err(error) => {
            let _ = send_outbound_msgpack(ws_session, &WsEvent::Error { message: format!("invalid message: {error}") }).await;
        }
    }
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

async fn send_outbound_json(
    ws_session: &mut Session,
    message: &WsEvent,
) -> Result<(), ()> {
    let json: String = serde_json::to_string(message)
        .map_err(|_| ())?;
    ws_session.text(json)
        .await
        .map_err(|_| ())
}

async fn send_outbound_msgpack(
    ws_session: &mut Session,
    message: &WsEvent,
) -> Result<(), ()> {
    let bytes: Vec<u8> = rmp_serde::to_vec_named(message)
        .map_err(|_| ())?;
    ws_session.binary(bytes)
        .await
        .map_err(|_| ())
}
