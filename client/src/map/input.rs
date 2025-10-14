use crate::input::{ClickResult, HoverResult};
use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::map::state::Hex;
use crate::map::HexCoord;
use crate::state::STATE;
use crate::window;
use crate::window::hex::HexWindow;
use crate::window::Window;
use raylib::math::Vector2;
use raylib::RaylibHandle;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use crate::window::state::WINDOW_LAYERS;

pub fn handle_click_map(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
    for window in WINDOW_LAYERS {
        let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
        match window.handle_click(rl, mouse_position) {
            ClickResult::Pass => continue,
            ClickResult::Consume => return ClickResult::Consume,
        }
    }

    handle_click_hex(rl, mouse_position)
}

pub fn handle_hover_map(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
    for window in WINDOW_LAYERS {
        let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
        match window.handle_hover(rl, mouse_position) {
            HoverResult::Pass => continue,
            HoverResult::Consume => return HoverResult::Consume,
        }
    }
    handle_hover_hex(rl, mouse_position)
}

fn handle_click_hex(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
    let containing_hex: Hex = {
        let map_origin: RwLockReadGuard<MapCoord> = STATE.stage.map.map_origin.read().unwrap();
        mouse_position.containing_hex(&*map_origin)
    };

    let mut hex_window: RwLockWriteGuard<HexWindow> = STATE.stage.map.window.hex.write().unwrap();
    hex_window.open(rl, RenderCoord(Vector2::from(mouse_position)), containing_hex);
    drop(hex_window);

    ClickResult::Consume
}

fn handle_hover_hex(_rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
    if window::any_window_open() {
        return HoverResult::Pass;
    }

    let containing_hex: Hex = {
        let map_origin: RwLockReadGuard<MapCoord> = STATE.stage.map.map_origin.read().unwrap();
        mouse_position.containing_hex(&*map_origin)
    };

    let mut hovered_hex_coord: RwLockWriteGuard<Option<HexCoord>> = STATE.stage.map.hovered_hex_coord.write().unwrap();
    *hovered_hex_coord = Some(containing_hex.hex_coord);

    HoverResult::Consume
}
