use actix_web::{HttpRequest, HttpResponse, rt, web};
use actix_ws::Message;
use std::time::{Duration, Instant};

use crate::lobby_error::LobbyError;
use crate::session::session_extractor::AuthenticatedAccount;

const RATE_LIMIT_MESSAGES_PER_SECOND: u32 = 10;
const RATE_LIMIT_INTERVAL: Duration = Duration::from_millis(1000 / RATE_LIMIT_MESSAGES_PER_SECOND as u64);

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(web::resource("/ws/live").route(web::get().to(live_ws_handler)));
}

async fn live_ws_handler(
    request: HttpRequest,
    body: web::Payload,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let account_id: uuid::Uuid = auth.account_id;

    let (response, mut ws_session, mut message_stream) = actix_ws::handle(&request, body)
        .map_err(|error| LobbyError::bad_request(&error.to_string()))?;

    log::info!("WebSocket live connection opened for account [{}]", account_id);

    rt::spawn(async move {
        let mut last_delivery: Instant = Instant::now();

        while let Some(message_result) = message_stream.recv().await {
            let message: Message = match message_result {
                Ok(message) => message,
                Err(error) => {
                    log::warn!("WebSocket receive error for account [{}]: {}", account_id, error);
                    break;
                }
            };

            match message {
                Message::Text(text) => {
                    // Rate limit outbound delivery
                    let elapsed: Duration = last_delivery.elapsed();
                    if elapsed < RATE_LIMIT_INTERVAL {
                        let sleep_duration: Duration = RATE_LIMIT_INTERVAL - elapsed;
                        rt::time::sleep(sleep_duration).await;
                    }
                    last_delivery = Instant::now();

                    log::debug!("WebSocket live text from account [{}]: {}", account_id, text);
                    // TODO: parse and route inbound messages
                    let echo: String = format!("echo: {}", text);
                    if ws_session.text(echo).await.is_err() {
                        break;
                    }
                }
                Message::Binary(bytes) => {
                    let elapsed: Duration = last_delivery.elapsed();
                    if elapsed < RATE_LIMIT_INTERVAL {
                        let sleep_duration: Duration = RATE_LIMIT_INTERVAL - elapsed;
                        rt::time::sleep(sleep_duration).await;
                    }
                    last_delivery = Instant::now();

                    log::debug!("WebSocket live binary from account [{}]: {} bytes", account_id, bytes.len());
                    // TODO: parse MessagePack inbound messages
                    if ws_session.binary(bytes).await.is_err() {
                        break;
                    }
                }
                Message::Ping(payload) => {
                    if ws_session.pong(&payload).await.is_err() {
                        break;
                    }
                }
                Message::Close(reason) => {
                    log::info!("WebSocket live connection closed for account [{}]", account_id);
                    let _ = ws_session.close(reason).await;
                    break;
                }
                _ => {}
            }
        }

        log::info!("WebSocket live task ended for account [{}]", account_id);
    });

    Ok(response)
}
