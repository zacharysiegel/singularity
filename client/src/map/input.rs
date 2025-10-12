use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::state::{Hex, STATE};
use crate::window::hex::HexWindow;
use raylib::math::Vector2;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub fn select_hex(mouse_position: RenderCoord) {
    let map_origin: RwLockReadGuard<MapCoord> = STATE.map_origin.read().unwrap();
    let containing_hex: Hex = mouse_position.containing_hex(&*map_origin);

    let mut hex_window: RwLockWriteGuard<HexWindow> = STATE.windows.hex.write().unwrap();
    hex_window.open(RenderCoord(Vector2::from(mouse_position)), containing_hex);
    drop(hex_window);
}
