use shared::schema::ws_message::ConnectionType;

use crate::conversation;

pub async fn after_connect(connection_type: ConnectionType, token: &str) {
    match connection_type {
        ConnectionType::Lobby => {
            conversation::catchup::catch_up(token).await;
        }
        ConnectionType::Live => {}
    }
}
