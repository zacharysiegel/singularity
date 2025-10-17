use crate::input::{ClickResult, HoverResult};
use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::map::state::Hex;
use crate::map::HexCoord;
use crate::state::STATE;
use crate::window;
use crate::window::HexWindow;
use raylib::math::Vector2;
use raylib::RaylibHandle;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub fn handle_click_hex(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
    let containing_hex: Hex = {
        let map_origin: RwLockReadGuard<MapCoord> = STATE.stage.game.map.map_origin.read().unwrap();
        mouse_position.containing_hex(&*map_origin)
    };

    let mut hex_window: RwLockWriteGuard<HexWindow> = STATE.stage.game.window.hex.write().unwrap();
    hex_window.open(rl, RenderCoord(Vector2::from(mouse_position)), containing_hex);
    drop(hex_window);

    ClickResult::Consume
}

pub fn handle_hover_hex(_rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
    if window::any_window_open() {
        return HoverResult::Pass;
    }

    let containing_hex: Hex = {
        let map_origin: RwLockReadGuard<MapCoord> = STATE.stage.game.map.map_origin.read().unwrap();
        mouse_position.containing_hex(&*map_origin)
    };

    let mut hovered_hex_coord: RwLockWriteGuard<Option<HexCoord>> =
        STATE.stage.game.map.hovered_hex_coord.write().unwrap();
    *hovered_hex_coord = Some(containing_hex.hex_coord);

    HoverResult::Consume
}
