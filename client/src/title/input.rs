use crate::button::RectangularButton;
use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use crate::map::RenderCoord;
use crate::state::STATE;
use raylib::RaylibHandle;
use std::sync::RwLockWriteGuard;

pub fn handle_click_title(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
    let mut debug_button: RwLockWriteGuard<Option<RectangularButton>> = STATE.stage.title.debug_button.write().unwrap();
    if debug_button.is_some() {
        if let ClickResult::Consume = debug_button.as_mut().unwrap().handle_click(rl, mouse_position) {
            return ClickResult::Consume;
        }
    }
    ClickResult::Pass
}

pub fn handle_hover_title(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
    let mut debug_button: RwLockWriteGuard<Option<RectangularButton>> = STATE.stage.title.debug_button.write().unwrap();
    if debug_button.is_some() {
        if let HoverResult::Consume = debug_button.as_mut().unwrap().handle_hover(rl, mouse_position) {
            return HoverResult::Consume;
        }
    }
    HoverResult::Pass
}
