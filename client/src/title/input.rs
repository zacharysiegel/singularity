use crate::component::button::RectangularButton;
use crate::component::vertical_scroll_region::VerticalScrollRegion;
use crate::input::{
    CharPressHandler, CharPressResult,
    ClickHandler, ClickResult, HoverHandler, HoverResult, KeyPressHandler, KeyPressResult,
    ScrollHandler, ScrollResult,
};
use crate::state::{Loading, STATE};
use crate::component::text_input::TextInput;
use raylib::consts::KeyboardKey;
use raylib::math::Vector2;
use raylib::RaylibHandle;
use shared::map::RenderCoord;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub struct TitleInput;

impl ClickHandler for TitleInput {
    fn click(&mut self, rl: &mut RaylibHandle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
        let mut debug_button: RwLockWriteGuard<Option<RectangularButton>> =
            STATE.stage.title.debug_button.button.write().unwrap();
        let debug_ready: RwLockReadGuard<Loading> = STATE.stage.title.debug_button.loading.read().unwrap();
        if debug_button.is_some() && *debug_ready == Loading::Complete {
            if let ClickResult::Consume = debug_button.as_mut().unwrap().click(rl, press_position, release_position) {
                return ClickResult::Consume;
            }
        }

        for button_l in &STATE.stage.title.main_buttons {
            let mut button: RwLockWriteGuard<RectangularButton> = button_l.write().unwrap();
            let result: ClickResult = button.click(rl, press_position, release_position);
            if let ClickResult::Consume = result {
                return ClickResult::Consume;
            }
        }

        let mut text_box_guard: RwLockWriteGuard<Option<TextInput>> = STATE.stage.title.debug_text_box.write().unwrap();
        if let Some(text_box) = text_box_guard.as_mut() {
            if let ClickResult::Consume = text_box.click(rl, press_position, release_position) {
                return ClickResult::Consume;
            }
        }

        ClickResult::Pass
    }
}

impl HoverHandler for TitleInput {
    fn hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        let mut debug_button: RwLockWriteGuard<Option<RectangularButton>> =
            STATE.stage.title.debug_button.button.write().unwrap();
        if debug_button.is_some() {
            if let HoverResult::Consume = debug_button.as_mut().unwrap().hover(rl, mouse_position) {
                return HoverResult::Consume;
            }
        }

        for button_l in &STATE.stage.title.main_buttons {
            let mut button: RwLockWriteGuard<RectangularButton> = button_l.write().unwrap();
            if let HoverResult::Consume = button.hover(rl, mouse_position) {
                return HoverResult::Consume;
            }
        }

        let mut text_box_guard: RwLockWriteGuard<Option<TextInput>> = STATE.stage.title.debug_text_box.write().unwrap();
        if let Some(text_box) = text_box_guard.as_mut() {
            if let HoverResult::Consume = text_box.hover(rl, mouse_position) {
                return HoverResult::Consume;
            }
        }

        HoverResult::Pass
    }
}

impl KeyPressHandler for TitleInput {
    fn key_press(&mut self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        let mut text_box_guard: RwLockWriteGuard<Option<TextInput>> = STATE.stage.title.debug_text_box.write().unwrap();
        if let Some(text_box) = text_box_guard.as_mut() {
            if let KeyPressResult::Consume = text_box.key_press(rl, key) {
                return KeyPressResult::Consume;
            }
        }
        KeyPressResult::Pass
    }
}

impl CharPressHandler for TitleInput {
    fn char_press(&mut self, rl: &mut RaylibHandle, ch: char) -> CharPressResult {
        let mut text_box_guard: RwLockWriteGuard<Option<TextInput>> = STATE.stage.title.debug_text_box.write().unwrap();
        if let Some(text_box) = text_box_guard.as_mut() {
            if let CharPressResult::Consume = text_box.char_press(rl, ch) {
                return CharPressResult::Consume;
            }
        }
        CharPressResult::Pass
    }
}

impl ScrollHandler for TitleInput {
    fn scroll(&mut self, rl: &mut RaylibHandle, scroll_v: Vector2, mouse_position: RenderCoord) -> ScrollResult {
        let mut scroll_guard: RwLockWriteGuard<Option<VerticalScrollRegion>> =
            STATE.stage.title.debug_scroll_region.write().unwrap();
        if let Some(scroll_region) = scroll_guard.as_mut() {
            if let ScrollResult::Consume = scroll_region.scroll(rl, scroll_v, mouse_position) {
                return ScrollResult::Consume;
            }
        }
        ScrollResult::Pass
    }
}
