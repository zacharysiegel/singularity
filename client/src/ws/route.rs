use std::sync::RwLockWriteGuard;
use shared::schema::ws_message::{ConnectionType, WsEvent};
use crate::conversation;
use crate::state::STATE;

pub fn route_ws_event(connection_type: ConnectionType, ws_event: WsEvent) {
    match ws_event {
        WsEvent::Chat(message) => {
            conversation::handle_message(message);
        }
        WsEvent::MemberJoined(change) => {
            conversation::handle_member_joined(change);
        }
        WsEvent::MemberLeft(change) => {
            conversation::handle_member_left(change);
        }
        WsEvent::Error { message } => {
            log::warn!("WsEvent::Error [{connection_type}]: {message}");
            let mut last_error: RwLockWriteGuard<Option<String>> = STATE.ws.last_error.write().unwrap();
            *last_error = Some(message);
        }
    }
}
