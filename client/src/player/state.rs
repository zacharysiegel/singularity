use std::sync::RwLock;
use crate::map::HexCoord;

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

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone)]
pub enum FacilityType {
    #[default]
    ControlCenter = 0,
    MetalExtractor,
    OilExtractor,
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone)]
pub enum FacilityState {
    #[default]
    Operating = 0,
    Placing,
    Destroyed,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Facility {
    pub location: HexCoord,
    pub facility_type: FacilityType,
    pub facility_state: FacilityState,
}
