use actix_ws::{CloseCode, CloseReason, ProtocolError, Session};
use uuid::Uuid;
use crate::ws::connection_type::ConnectionType;

pub async fn handle_receive_error(
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

pub async fn handle_ping(
    ws_session: &mut Session,
    connection_type: ConnectionType,
    account_id: Uuid,
    payload: &[u8],
) -> bool {
    log::trace!("WebSocket PING [{connection_type}] [{account_id}]");
    ws_session.pong(payload).await.is_ok()
}

pub async fn handle_close(
    ws_session: Session,
    connection_type: ConnectionType,
    account_id: Uuid,
    reason: Option<CloseReason>,
) -> bool {
    log::debug!("WebSocket CLOSE [{connection_type}] [{account_id}]");
    let _ = ws_session.close(reason).await;
    false
}
