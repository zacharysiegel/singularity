use crate::map;
use crate::map::coordinate::RenderCoord;
use crate::window::{Window, WINDOW_LAYERS};
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
    for window in WINDOW_LAYERS {
        let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
        match window.handle_click(rl, mouse_position) {
            ClickResult::Pass => {}
            ClickResult::Consume => return,
        }
    }
    map::click_map(rl, mouse_position);
}

fn hover(rl: &mut RaylibHandle, mouse_position: RenderCoord) {
    for window in WINDOW_LAYERS {
        let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
        match window.handle_hover(rl, mouse_position) {
            HoverResult::Pass => {}
            HoverResult::Consume => return,
        }
    }
    map::hover_map(rl, mouse_position);
}
