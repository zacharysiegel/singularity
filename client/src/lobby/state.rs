use std::sync::RwLock;

#[derive(Debug)]
pub struct LobbyState {
    /// Bearer token issued by the lobby's `POST /session` endpoint. Set after authentication
    /// succeeds. Used for both HTTP requests to the lobby (`Authorization: Bearer ...`) and
    /// the lobby/live WebSocket upgrade requests.
    pub token: RwLock<Option<String>>,
}

impl LobbyState {
    pub fn new() -> Self {
        LobbyState {
            token: RwLock::new(None),
        }
    }
}
