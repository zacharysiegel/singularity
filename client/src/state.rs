use crate::map::coordinate::HexCoord;
use crate::map::MapState;
use crate::player::state::PlayerState;
use crate::stage::StageState;
use crate::window::WindowState;
use std::sync::RwLock;

pub static STATE: State = State {
    frame_counter: RwLock::new(0),
    stage: StageState::DEFAULT,
    player: PlayerState::DEFAULT,
    window: WindowState::DEFAULT,
    map: MapState::DEFAULT,
};

#[derive(Debug)]
pub struct State {
    pub frame_counter: RwLock<u64>,
    pub stage: StageState,
    pub player: PlayerState,
    pub map: MapState,
    pub window: WindowState,
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
