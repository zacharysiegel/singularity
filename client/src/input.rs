use crate::map;
use crate::map::coordinate::RenderCoord;
use crate::window::{Window, WINDOW_LAYERS};
use raylib::consts::MouseButton;
use raylib::RaylibHandle;
use std::sync::RwLockWriteGuard;

pub enum ClickResult {
    Pass,
    Consume,
}

pub trait Clickable {
    /// Hook to allow an object to handle a click event.
    /// The hook should return [ClickResult::Consume] to consume the event, or
    /// [ClickResult::Pass] to allow subsequent objects to handle the same event.
    fn handle_click(&mut self, mouse_position: RenderCoord) -> ClickResult;
}

pub fn handle_user_input(rl: &mut RaylibHandle) {
    if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
        click(RenderCoord(rl.get_mouse_position()));
    }
}

fn click(mouse_position: RenderCoord) {
    for window in WINDOW_LAYERS {
        let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
        match window.handle_click(mouse_position) {
            ClickResult::Pass => {}
            ClickResult::Consume => return,
        }
    }
    map::select_hex(mouse_position);
}
