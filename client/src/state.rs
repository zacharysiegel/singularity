use crate::config;
use crate::config::{HEX_COUNT_SQRT, HEX_HEIGHT, HEX_RADIUS, HEX_SIDE_LENGTH};
use crate::map::MapCoord;
use raylib::color::Color;
use raylib::ffi::Vector2;
use std::sync::RwLock;

// todo: separate locks per field, or just separate bindings
pub static STATE: RwLock<State> = RwLock::new(State {
    frame_counter: 0,
    map_origin: MapCoord::DEFAULT,
    hexes: [Hex::DEFAULT; config::HEX_COUNT as usize],
    players: Vec::new(),
});

#[derive(Debug, Copy, Clone)]
pub struct HexCoord {
    pub i: u16,
    pub j: u16,
}

impl Default for HexCoord {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl HexCoord {
    pub const DEFAULT: HexCoord = HexCoord { i: 0, j: 0 };

    pub fn clone_map_hex(&self) -> Option<Hex> {
        let state = STATE.read().expect("global state poisoned");
        state.hexes.get(self.map_index()).map(|hex| hex.clone())
    }

    pub const fn map_index(&self) -> usize {
        (self.i + self.j * HEX_COUNT_SQRT) as usize
    }

    pub fn map_coord(&self) -> MapCoord {
        let even_row: bool = self.j % 2 == 0;
        let x: f32 = (f32::from(self.i) * *HEX_HEIGHT)
            + (if even_row { 0_f32 } else { *HEX_HEIGHT / 2_f32 });
        let y: f32 = f32::from(self.j) * f32::from(HEX_RADIUS + HEX_SIDE_LENGTH / 2);
        MapCoord(Vector2 { x, y })
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
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
            ResourceType::None => Color {
                r: 0x00,
                g: 0x00,
                b: 0x00,
                a: 0x00,
            },
            ResourceType::Metal => config::METAL_BACKGROUND_COLOR,
            ResourceType::Oil => config::OIL_BACKGROUND_COLOR,
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
    pub frame_counter: u64,
    pub map_origin: MapCoord,
    pub hexes: [Hex; config::HEX_COUNT as usize],
    pub players: Vec<Player>,
}
