use crate::facility::Facility;
use crate::map::HexCoord;
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

#[derive(Debug, Default)]
pub struct Player {
    pub id: u8,
    pub facilities: Vec<Facility>, // todo: split to vector per facility type (avoids frequent filtering)
}

impl Player {
    pub fn new(id: u8) -> Self {
        Player {
            id,
            facilities: Vec::new(),
        }
    }

    pub fn within_influence(&self, hex_coord: HexCoord) -> bool {
        for facility in &self.facilities {
            match facility {
                Facility::ControlCenter(facility) => match facility.within_influence(hex_coord) {
                    true => return true,
                    false => continue,
                },
                _ => continue,
            }
        }
        false
    }
}
