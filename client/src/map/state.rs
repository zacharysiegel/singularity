use crate::color::{MAP_BACKGROUND_COLOR, METAL_BACKGROUND_COLOR, OIL_BACKGROUND_COLOR};
use crate::map;
use crate::map::{HexCoord, MapCoord};
use crate::player::state::PlayerState;
use crate::window::WindowState;
use raylib::color::Color;
use std::sync::RwLock;

#[derive(Debug)]
pub struct MapState {
    pub map_origin: RwLock<MapCoord>,
    pub hexes: RwLock<[Hex; map::config::HEX_COUNT as usize]>,
    pub hovered_hex_coord: RwLock<Option<HexCoord>>,
    pub player: PlayerState,
    pub window: WindowState,
}

impl MapState {
    pub const DEFAULT: MapState = MapState {
        map_origin: RwLock::new(MapCoord::DEFAULT),
        hexes: RwLock::new([Hex::DEFAULT; map::config::HEX_COUNT as usize]),
        hovered_hex_coord: RwLock::new(None),
        player: PlayerState::DEFAULT,
        window: WindowState::DEFAULT,
    };
}

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
