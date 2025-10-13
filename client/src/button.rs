use crate::input;
use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use crate::map::RenderCoord;
use raylib::math::Rectangle;
use raylib::RaylibHandle;

#[derive(Debug)]
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

impl Default for RectangularButton {
    fn default() -> RectangularButton {
        RectangularButton::DEFAULT
    }
}

impl RectangularButton {
    pub const DEFAULT: RectangularButton = RectangularButton {
        rectangle: Rectangle {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
        on_click: input::noop_on_click,
        on_hover: input::noop_on_hover,
        hovered: false,
    };

    pub fn new(rectangle: Rectangle) -> RectangularButton {
        let mut button: RectangularButton = RectangularButton::default();
        button.rectangle = rectangle;
        button
    }

    pub fn is_hovered(&self) -> bool {
        self.hovered
    }
}
