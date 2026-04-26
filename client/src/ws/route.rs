use shared::schema::ws_message::{ConnectionType, WsEvent};
use crate::conversation::event;
use crate::state::STATE;

pub fn route_ws_event(connection_type: ConnectionType, event: WsEvent) {
    match event {
        WsEvent::Chat { message } => {
            event::handle_chat_event(message);
        }
        WsEvent::MemberJoined { conversation_id, account_id } => {
            event::handle_member_joined(conversation_id, account_id);
        }
        WsEvent::MemberLeft { conversation_id, account_id } => {
            event::handle_member_left(conversation_id, account_id);
        }
        WsEvent::Error { message } => {
            log::warn!("WsEvent::Error [{connection_type}]: {message}");
            let mut last_error = STATE.ws.last_error.write().unwrap();
            *last_error = Some(message);
        }
    }
}
