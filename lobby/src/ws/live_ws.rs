use actix_web::{rt, web, HttpRequest, HttpResponse};
use actix_ws::Message;
use sqlx::PgPool;
use std::time::{Duration, Instant};

use crate::http;
use crate::session::session_db;

const RATE_LIMIT_MESSAGES_PER_SECOND: u32 = 10;
const RATE_LIMIT_INTERVAL: Duration = Duration::from_millis(1000 / RATE_LIMIT_MESSAGES_PER_SECOND as u64);

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(web::resource("/ws/live").route(web::get().to(live_ws_handler)));
}

async fn live_ws_handler(
    request: HttpRequest,
    body: web::Payload,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let token: &str = http::extract_bearer_token(&request)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("missing or invalid authorization header"))?;

    let session_entity = session_db::get_session_by_token(pool.get_ref(), token)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("db error"))?
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("invalid or expired session"))?;

    let account_id: uuid::Uuid = session_entity.account_id;

    let (response, mut ws_session, mut message_stream) = actix_ws::handle(&request, body)?;

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
