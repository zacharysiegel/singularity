use crate::input::{
    CharPressHandler, CharPressResult, ClickButton, ClickHandler, ClickResult, HoverHandler, HoverResult,
    KeyPressHandler, KeyPressResult, ScrollHandler, ScrollResult,
};
use crate::map::MapInput;
use crate::state::STATE;
use crate::window::{PauseWindow, Window, window_layers};
use raylib::RaylibHandle;
use raylib::consts::KeyboardKey;
use raylib::math::Vector2;
use shared::map::RenderCoord;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub struct GameInput;

impl ScrollHandler for GameInput {
    fn scroll(&mut self, rl: &mut RaylibHandle, scroll_v: Vector2, mouse_position: RenderCoord) -> ScrollResult {
        for window in window_layers() {
            let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
            match window.scroll(rl, scroll_v, mouse_position) {
                ScrollResult::Pass => continue,
                ScrollResult::Consume => return ScrollResult::Consume,
            }
        }

        MapInput.scroll(rl, scroll_v, mouse_position)
    }
}

impl ClickHandler for GameInput {
    fn click(&mut self, rl: &mut RaylibHandle, button: ClickButton, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
        for window in window_layers() {
            let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
            match window.click(rl, button, press_position, release_position) {
                ClickResult::Pass => continue,
                ClickResult::Consume => return ClickResult::Consume,
            }
        }

        MapInput.click(rl, button, press_position, release_position)
    }
}

impl HoverHandler for GameInput {
    fn hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        for window in window_layers() {
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
        MapInput.hover(rl, mouse_position)
    }
}

impl KeyPressHandler for GameInput {
    fn key_press(&mut self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        if key == KeyboardKey::KEY_P {
            let pause_window: RwLockReadGuard<PauseWindow> = STATE.stage.game.window.pause.read().unwrap();
            if !pause_window.is_open() {
                drop(pause_window);
                let mut pause_window: RwLockWriteGuard<PauseWindow> = STATE.stage.game.window.pause.write().unwrap();
                pause_window.open(rl);
                return KeyPressResult::Consume;
            }
        }

        for window in window_layers() {
            let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
            match window.key_press(rl, key) {
                KeyPressResult::Pass => continue,
                KeyPressResult::Consume => return KeyPressResult::Consume,
            }
        }

        KeyPressResult::Pass
    }
}

impl CharPressHandler for GameInput {
    fn char_press(&mut self, rl: &mut RaylibHandle, ch: char) -> CharPressResult {
        for window in window_layers() {
            let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
            match window.char_press(rl, ch) {
                CharPressResult::Pass => continue,
                CharPressResult::Consume => return CharPressResult::Consume,
            }
        }

        CharPressResult::Pass
    }
}
