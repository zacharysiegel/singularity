use crate::input::HoverResult;
use crate::input::KeyPressResult;
use crate::input::{ClickResult, ScrollResult};
use crate::map;
use crate::map::{MapCoord, RenderCoord};
use crate::state::STATE;
use crate::window::{PauseWindow, WINDOW_LAYERS, Window};
use raylib::RaylibHandle;
use raylib::consts::KeyboardKey;
use raylib::math::Vector2;
use std::ops::{Add, Mul};
use std::sync::RwLockWriteGuard;

pub fn scroll(_rl: &mut RaylibHandle, scroll_v: Vector2) -> ScrollResult {
    let mut map_origin: RwLockWriteGuard<MapCoord> =
        STATE.stage.game.map.map_origin.write().expect("global state poisoned");

    *map_origin = scrolled_map_origin(*map_origin, scroll_v);
    ScrollResult::Consume
}

pub fn click(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
    for window in WINDOW_LAYERS {
        let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
        match window.click(rl, mouse_position) {
            ClickResult::Pass => continue,
            ClickResult::Consume => return ClickResult::Consume,
        }
    }

    map::handle_click_hex(rl, mouse_position)
}

pub fn hover(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
    for window in WINDOW_LAYERS {
        let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
        match window.hover(rl, mouse_position) {
            HoverResult::Pass => continue,
            HoverResult::Consume => return HoverResult::Consume,
        }
    }
    map::handle_hover_hex(rl, mouse_position)
}

pub fn key_press(rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
    let mut pause_window: RwLockWriteGuard<PauseWindow> = STATE.stage.game.window.pause.write().unwrap();
    if pause_window.is_open() {
        if key == KeyboardKey::KEY_P {
            pause_window.close();
            return KeyPressResult::Consume;
        }
        return KeyPressResult::Pass;
    }
    if key == KeyboardKey::KEY_P {
        pause_window.open(rl);
        return KeyPressResult::Consume;
    }
    drop(pause_window);

    if key == KeyboardKey::KEY_ESCAPE {
        let mut hex_window = STATE.stage.game.window.hex.write().unwrap();
        if hex_window.is_open() {
            hex_window.close();
        }
        return KeyPressResult::Consume;
    }
    KeyPressResult::Pass
}

fn scrolled_map_origin(map_origin: MapCoord, scroll_v: Vector2) -> MapCoord {
    let scroll_inverted: Vector2 = scroll_v.mul(Vector2 { x: -1., y: -1. });
    let unchecked_origin: Vector2 = map_origin.add(scroll_inverted);
    MapCoord(unchecked_origin).overflow_adjusted()
}
