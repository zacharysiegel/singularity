use std::sync::RwLockWriteGuard;
use raylib::RaylibHandle;
use crate::input::{ClickResult, HoverResult};
use crate::map::RenderCoord;
use crate::state::STATE;

pub fn handle_click_title(_rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> ClickResult {
    let mut current_i: RwLockWriteGuard<usize> = STATE.stage.current_index.write().unwrap();
    *current_i = 1;

    ClickResult::Consume
}

pub fn handle_hover_title(_rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> HoverResult {
    HoverResult::Consume
}
