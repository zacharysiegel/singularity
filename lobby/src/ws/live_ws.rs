use crate::lobby_error::{LobbyError, ResultExtLobbyError};
use crate::session::session_extractor::AuthenticatedAccount;
use actix_web::{rt, web, HttpRequest, HttpResponse};
use actix_ws::{Message, MessageStream, Session};
use bytes::Bytes;
use bytestring::ByteString;
use std::time::{Duration, Instant};
use uuid::Uuid;
use crate::ws::connection_type::ConnectionType;
use crate::ws::frame;

const CONNECTION_TYPE: ConnectionType = ConnectionType::Live;
/// The "live" WebSocket connection is used by the client while in-game. To avoid frame rate spikes
/// or denial of service, we enforce a rate limit on outbound messages through this socket.
const RATE_LIMIT_MESSAGES_PER_SECOND: u64 = 10;
const RATE_LIMIT_INTERVAL: Duration = Duration::from_millis(1000 / RATE_LIMIT_MESSAGES_PER_SECOND);

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(web::resource(format!("/ws/{CONNECTION_TYPE}")).route(web::get().to(live_ws_handler)));
}

async fn live_ws_handler(
    request: HttpRequest,
    body: web::Payload,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let account_id: Uuid = auth.account_id;

    let (upgrade_response, mut ws_session, mut message_stream): (HttpResponse, Session, MessageStream) =
        actix_ws::handle(&request, body).or_bad_request()?;

    rt::spawn(async move {
        let mut last_delivery: Instant = Instant::now();

        while let Some(message_result) = message_stream.recv().await {
            let message: Message = match message_result {
                Ok(message) => message,
                Err(error) => {
                    frame::handle_receive_error(ws_session.clone(), CONNECTION_TYPE, account_id, error).await;
                    break;
                }
            };

            let should_continue: bool = match message {
                Message::Text(text) => handle_text(&mut ws_session, account_id, text, &mut last_delivery).await,
                Message::Binary(bytes) => handle_binary(&mut ws_session, account_id, bytes, &mut last_delivery).await,
                Message::Ping(payload) => frame::handle_ping(&mut ws_session, CONNECTION_TYPE, account_id, &payload).await,
                Message::Close(reason) => frame::handle_close(ws_session.clone(), CONNECTION_TYPE, account_id, reason).await,
                _ => true,
            };

            if !should_continue {
                break;
            }
        }

        log::info!("WebSocket task ended [{CONNECTION_TYPE}] [{account_id}]");
    });

    log::info!("WebSocket connection opened [{CONNECTION_TYPE}] [{account_id}]");
    Ok(upgrade_response)
}

async fn throttle(last_delivery: &mut Instant) {
    let elapsed: Duration = last_delivery.elapsed();
    if elapsed < RATE_LIMIT_INTERVAL {
        let sleep_duration: Duration = RATE_LIMIT_INTERVAL - elapsed;
        rt::time::sleep(sleep_duration).await;
    }
    *last_delivery = Instant::now();
}

async fn handle_text(
    ws_session: &mut Session,
    account_id: Uuid,
    text: ByteString,
    last_delivery: &mut Instant,
) -> bool {
    throttle(last_delivery).await;
    log::trace!(
        "WebSocket TEXT [{CONNECTION_TYPE}] [{account_id}]: {} bytes",
        text.as_bytes().len(),
    );
    // TODO: parse and route inbound messages
    let echo: String = format!("echo: {}", text);
    ws_session.text(echo).await.is_ok()
}

async fn handle_binary(
    ws_session: &mut Session,
    account_id: Uuid,
    bytes: Bytes,
    last_delivery: &mut Instant,
) -> bool {
    throttle(last_delivery).await;
    log::trace!(
        "WebSocket BINARY [{CONNECTION_TYPE}] [{account_id}]: {} bytes",
        bytes.len(),
    );
    // TODO: parse MessagePack inbound messages
    ws_session.binary(bytes).await.is_ok()
}
