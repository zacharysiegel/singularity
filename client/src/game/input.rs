use crate::input::ClickResult;
use crate::input::HoverResult;
use crate::input::KeyPressResult;
use crate::map;
use crate::map::RenderCoord;
use crate::state::STATE;
use crate::window::{PauseWindow, Window, WINDOW_LAYERS};
use raylib::consts::KeyboardKey;
use raylib::RaylibHandle;
use std::sync::RwLockWriteGuard;

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
    if key == KeyboardKey::KEY_ESCAPE {
        let mut pause_window: RwLockWriteGuard<PauseWindow> = STATE.stage.game.window.pause.write().unwrap();
        if pause_window.is_open() {
            pause_window.close();
        } else {
            pause_window.open(rl);
        }
        return KeyPressResult::Consume;
    }
    KeyPressResult::Pass
}
