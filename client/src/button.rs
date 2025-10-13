use crate::input;
use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use crate::map::RenderCoord;
use raylib::math::Rectangle;
use raylib::RaylibHandle;

pub struct RectangularButton {
    pub rectangle: Rectangle,
    pub on_click: fn(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult,
    pub on_hover: fn(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult,

    hovered: bool,
}

impl ClickHandler for RectangularButton {
    fn handle_click(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
        if self.rectangle.check_collision_point_rec(mouse_position) {
            (self.on_click)(rl, mouse_position)
        } else {
            ClickResult::Pass
        }
    }
}

impl HoverHandler for RectangularButton {
    fn handle_hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        if self.rectangle.check_collision_point_rec(mouse_position) {
            self.hovered = true;
            (self.on_hover)(rl, mouse_position)
        } else {
            HoverResult::Pass
        }
    }
}

impl RectangularButton {
    pub fn new(rectangle: Rectangle) -> RectangularButton {
        RectangularButton {
            rectangle,
            on_click: input::noop_on_click,
            on_hover: input::noop_on_hover,
            hovered: false,
        }
    }
}
