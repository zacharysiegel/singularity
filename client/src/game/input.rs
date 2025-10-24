use crate::input::HoverResult;
use crate::input::KeyPressResult;
use crate::input::{ClickResult, ScrollResult};
use crate::map;
use crate::map::RenderCoord;
use crate::state::STATE;
use crate::window::{PauseWindow, Window, WINDOW_LAYERS};
use raylib::consts::KeyboardKey;
use raylib::math::Vector2;
use raylib::RaylibHandle;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub fn scroll(rl: &mut RaylibHandle, scroll_v: Vector2) -> ScrollResult {
    for window in WINDOW_LAYERS {
        let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
        match window.scroll(rl, scroll_v) {
            ScrollResult::Pass => continue,
            ScrollResult::Consume => return ScrollResult::Consume,
        }
    }

    map::scroll(rl, scroll_v)
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
            HoverResult::Pass => {
                if window.is_open() {
                    return HoverResult::Consume;
                } else {
                    continue;
                }
            }
            HoverResult::Consume => return HoverResult::Consume,
        }
    }
    map::handle_hover_hex(rl, mouse_position)
}

pub fn key_press(rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
    if key == KeyboardKey::KEY_P {
        let pause_window: RwLockReadGuard<PauseWindow> = STATE.stage.game.window.pause.read().unwrap();
        if !pause_window.is_open() {
            drop(pause_window);
            let mut pause_window: RwLockWriteGuard<PauseWindow> = STATE.stage.game.window.pause.write().unwrap();
            pause_window.open(rl);
            return KeyPressResult::Consume;
        }
    }

    for window in WINDOW_LAYERS {
        let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
        match window.key_press(rl, key) {
            KeyPressResult::Pass => continue,
            KeyPressResult::Consume => return KeyPressResult::Consume,
        }
    }

    KeyPressResult::Pass
}
