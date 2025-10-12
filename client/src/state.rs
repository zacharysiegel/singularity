use crate::color::{MAP_BACKGROUND_COLOR, METAL_BACKGROUND_COLOR, OIL_BACKGROUND_COLOR};
use crate::map;
use crate::map::coordinate::{HexCoord, MapCoord};
use crate::window::WindowState;
use raylib::color::Color;
use std::sync::RwLock;

pub static STATE: State = State {
    frame_counter: RwLock::new(0),
    map_origin: RwLock::new(MapCoord::DEFAULT),
    hexes: RwLock::new([Hex::DEFAULT; map::config::HEX_COUNT as usize]),
    players: RwLock::new(Vec::new()),
    windows: WindowState::DEFAULT,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ResourceType {
    None = 0,
    Metal,
    Oil,
}

impl Default for ResourceType {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl ResourceType {
    pub const DEFAULT: ResourceType = ResourceType::None;

    pub const fn color(&self) -> Color {
        match self {
            ResourceType::None => MAP_BACKGROUND_COLOR,
            ResourceType::Metal => METAL_BACKGROUND_COLOR,
            ResourceType::Oil => OIL_BACKGROUND_COLOR,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Hex {
    pub hex_coord: HexCoord,
    pub resource_type: ResourceType,
}

impl Default for Hex {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Hex {
    pub const DEFAULT: Hex = Hex {
        hex_coord: HexCoord::DEFAULT,
        resource_type: ResourceType::DEFAULT,
    };
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

#[derive(Debug)]
pub struct State {
    pub frame_counter: RwLock<u64>,
    pub map_origin: RwLock<MapCoord>,
    pub hexes: RwLock<[Hex; map::config::HEX_COUNT as usize]>,
    pub players: RwLock<Vec<Player>>,
    pub windows: WindowState,
}
