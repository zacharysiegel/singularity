use crate::input::{ClickResult, HoverResult};
use crate::map;
use crate::map::RenderCoord;
use crate::window::{Window, WINDOW_LAYERS};
use raylib::RaylibHandle;
use std::sync::RwLockWriteGuard;

pub fn handle_click_title(rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> ClickResult {
    ClickResult::Consume
}

pub fn handle_click_map(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
    for window in WINDOW_LAYERS {
        let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
        match window.handle_click(rl, mouse_position) {
            ClickResult::Pass => continue,
            ClickResult::Consume => return ClickResult::Consume,
        }
    }

    map::click_map(rl, mouse_position)
}
pub fn handle_hover_title(rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> HoverResult {
    HoverResult::Consume
}

pub fn handle_hover_map(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
    for window in WINDOW_LAYERS {
        let mut window: RwLockWriteGuard<dyn Window> = window.write().unwrap();
        match window.handle_hover(rl, mouse_position) {
            HoverResult::Pass => continue,
            HoverResult::Consume => return HoverResult::Consume,
        }
    }
    map::hover_map(rl, mouse_position)
}
