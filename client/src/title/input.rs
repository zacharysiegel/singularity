use crate::button::RectangularButton;
use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use crate::state::{Loading, STATE};
use raylib::RaylibHandle;
use shared::map::RenderCoord;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub fn click(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
    let mut debug_button: RwLockWriteGuard<Option<RectangularButton>> =
        STATE.stage.title.debug_button.button.write().unwrap();
    let debug_ready: RwLockReadGuard<Loading> = STATE.stage.title.debug_button.loading.read().unwrap();
    if debug_button.is_some() && *debug_ready == Loading::Complete {
        if let ClickResult::Consume = debug_button.as_mut().unwrap().click(rl, mouse_position) {
            return ClickResult::Consume;
        }
    }

    for button_l in &STATE.stage.title.main_buttons {
        let mut button: RwLockWriteGuard<RectangularButton> = button_l.write().unwrap();
        let result: ClickResult = button.click(rl, mouse_position);
        if let ClickResult::Consume = result {
            return ClickResult::Consume;
        }
    }
    ClickResult::Pass
}

pub fn hover(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
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
    HoverResult::Pass
}
