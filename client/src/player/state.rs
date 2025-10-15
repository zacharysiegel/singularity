use crate::facility::Facility;
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

#[derive(Debug, Default)]
pub struct Player {
    pub id: u8,
    pub facilities: Vec<Facility>,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Player {
            id,
            facilities: Vec::new(),
        }
    }
}
