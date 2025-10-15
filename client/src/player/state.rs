use crate::state::Player;
use std::sync::RwLock;

#[derive(Debug)]
pub struct PlayerState {
    pub players: RwLock<Vec<Player>>,
}

impl PlayerState {
    pub const DEFAULT: PlayerState = PlayerState {
        players: RwLock::new(Vec::new()),
    };
}
