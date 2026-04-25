use actix_web::{HttpRequest, HttpResponse, rt, web};
use actix_ws::{Message, MessageStream, Session};

use crate::lobby_error::{LobbyError, ResultExtLobbyError};
use crate::session::session_extractor::AuthenticatedAccount;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(web::resource("/ws/lobby").route(web::get().to(lobby_ws_handler)));
}

async fn lobby_ws_handler(
    request: HttpRequest,
    body: web::Payload,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let account_id: uuid::Uuid = auth.account_id;

    let (upgrade_response, mut ws_session, mut message_stream): (HttpResponse, Session, MessageStream) =
        actix_ws::handle(&request, body).or_bad_request()?;

    log::info!("WebSocket lobby connection opened for account [{}]", account_id);

    rt::spawn(async move {
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
                    log::debug!("WebSocket text from account [{}]: {}", account_id, text);
                    // TODO: parse and route inbound messages (SendMessage, etc.)
                    let echo: String = format!("echo: {}", text);
                    if ws_session.text(echo).await.is_err() {
                        break;
                    }
                }
                Message::Binary(bytes) => {
                    log::debug!("WebSocket binary from account [{}]: {} bytes", account_id, bytes.len());
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
                    log::info!("WebSocket lobby connection closed for account [{}]", account_id);
                    let _ = ws_session.close(reason).await;
                    break;
                }
                _ => {}
            }
        }

        log::info!("WebSocket lobby task ended for account [{}]", account_id);
    });

    Ok(upgrade_response)
}
