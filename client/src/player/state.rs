use shared::player::Player;
use std::sync::RwLock;

#[derive(Debug)]
pub struct PlayerState {
    pub players: RwLock<Vec<Player>>,
    pub selected: RwLock<usize>,
}

impl PlayerState {
    pub const DEFAULT: PlayerState = PlayerState {
        players: RwLock::new(Vec::new()),
        selected: RwLock::new(1),
    };
}
