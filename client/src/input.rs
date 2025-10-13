use crate::map::coordinate::RenderCoord;
use crate::stage::Stage;
use crate::state::STATE;
use raylib::consts::MouseButton;
use raylib::RaylibHandle;
use std::sync::RwLockWriteGuard;

#[derive(PartialEq)]
pub enum ClickResult {
    Pass,
    Consume,
}

pub trait ClickHandler {
    /// Hook to allow an object to handle a click event.
    /// The hook should return [ClickResult::Consume] to consume the event, or
    /// [ClickResult::Pass] to allow subsequent objects to handle the same event.
    fn handle_click(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult;
}

#[derive(PartialEq)]
pub enum HoverResult {
    Pass,
    Consume,
}

pub trait HoverHandler {
    /// Hook to allow an object to handle a mouse hover event.
    /// The hook should return [ClickResult::Consume] to consume the event, or
    /// [ClickResult::Pass] to allow subsequent objects to handle the same event.
    fn handle_hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult;
}

pub fn handle_user_input(rl: &mut RaylibHandle) {
    let mouse_position: RenderCoord = RenderCoord(rl.get_mouse_position());

    hover(rl, mouse_position);
    if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
        click(rl, mouse_position);
    }
}

fn click(rl: &mut RaylibHandle, mouse_position: RenderCoord) {
    let mut stage: RwLockWriteGuard<Stage> = STATE.stage.get_current_write();
    stage.handle_click(rl, mouse_position);
}

fn hover(rl: &mut RaylibHandle, mouse_position: RenderCoord) {
    let mut stage: RwLockWriteGuard<Stage> = STATE.stage.get_current_write();
    stage.handle_hover(rl, mouse_position);
}
