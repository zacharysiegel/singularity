use crate::state::STATE;
use shared::map::{HEX_COUNT, Hex, HexCoord, MapCoord};
use std::sync::RwLock;

#[derive(Debug)]
pub struct MapState {
    pub map_origin: RwLock<MapCoord>,
    pub hexes: RwLock<[Hex; HEX_COUNT as usize]>,
    pub hovered_hex_coord: RwLock<Option<HexCoord>>,
}

impl MapState {
    pub const DEFAULT: MapState = MapState {
        map_origin: RwLock::new(MapCoord::DEFAULT),
        hexes: RwLock::new([Hex::DEFAULT; HEX_COUNT as usize]),
        hovered_hex_coord: RwLock::new(None),
    };
}

pub fn clone_hex(hex_coord: HexCoord) -> Option<Hex> {
    let hexes = STATE.stage.game.map.hexes.read().expect("global state poisoned");
    hexes.get(hex_coord.map_index()).map(|hex| hex.clone())
}
