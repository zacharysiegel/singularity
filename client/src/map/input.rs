use crate::input::{ClickResult, HoverResult, ScrollResult};
use crate::map::HexCoord;
use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::map::state::Hex;
use crate::state::STATE;
use crate::window;
use crate::window::HexWindow;
use raylib::RaylibHandle;
use raylib::math::Vector2;
use std::ops::{Add, Mul};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub fn scroll(_rl: &mut RaylibHandle, scroll_v: Vector2) -> ScrollResult {
    let mut map_origin: RwLockWriteGuard<MapCoord> =
        STATE.stage.game.map.map_origin.write().expect("global state poisoned");

    *map_origin = scrolled_map_origin(*map_origin, scroll_v);
    ScrollResult::Consume
}

fn scrolled_map_origin(map_origin: MapCoord, scroll_v: Vector2) -> MapCoord {
    let scroll_inverted: Vector2 = scroll_v.mul(Vector2 { x: -1., y: -1. });
    let unchecked_origin: Vector2 = map_origin.add(scroll_inverted);
    MapCoord(unchecked_origin).overflow_adjusted()
}

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
