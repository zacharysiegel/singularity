use crate::input::{ClickResult, HoverResult};
use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::map::state::Hex;
use crate::state::STATE;
use crate::window;
use crate::window::hex::HexWindow;
use raylib::math::Vector2;
use raylib::RaylibHandle;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub fn click_map(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
    let map_origin: RwLockReadGuard<MapCoord> = STATE.map.map_origin.read().unwrap();
    let containing_hex: Hex = mouse_position.containing_hex(&*map_origin);

    let mut hex_window: RwLockWriteGuard<HexWindow> = STATE.window.hex.write().unwrap();
    hex_window.open(rl, RenderCoord(Vector2::from(mouse_position)), containing_hex);
    drop(hex_window);

    ClickResult::Consume
}

pub fn hover_map(_rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> HoverResult {
    HoverResult::Consume
}

pub fn map_has_focus() -> bool {
    !window::any_window_open()
}
