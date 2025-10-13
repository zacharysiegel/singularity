use crate::button::RectangularButton;
use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use crate::map::RenderCoord;
use crate::state::STATE;
use raylib::RaylibHandle;
use std::sync::RwLockWriteGuard;

pub fn handle_click_title(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
    let mut debug_button: RwLockWriteGuard<RectangularButton> = STATE.stage.title.debug_button.write().unwrap();
    if let ClickResult::Consume = debug_button.handle_click(rl, mouse_position) {
        return ClickResult::Consume;
    }
    ClickResult::Pass
}

pub fn handle_hover_title(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
    let mut debug_button: RwLockWriteGuard<RectangularButton> = STATE.stage.title.debug_button.write().unwrap();
    if let HoverResult::Consume = debug_button.handle_hover(rl, mouse_position) {
        return HoverResult::Consume;
    }
    HoverResult::Pass
}
