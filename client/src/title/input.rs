use crate::input::{noop_on_click, noop_on_hover, ClickResult, HoverResult};
use crate::map::RenderCoord;
use raylib::RaylibHandle;

pub fn handle_click_title(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
    noop_on_click(rl, mouse_position)
}

pub fn handle_hover_title(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
    noop_on_hover(rl, mouse_position)
}
