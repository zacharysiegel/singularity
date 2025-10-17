use crate::facility::FacilityCollection;
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
    pub facilities: FacilityCollection,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Player {
            id,
            facilities: FacilityCollection::default(),
        }
    }

    pub fn within_influence(&self, hex_coord: HexCoord) -> bool {
        for facility in &self.facilities.control_center_vec {
            match facility.within_influence(hex_coord) {
                true => return true,
                false => continue,
            }
        }
        false
    }
}
