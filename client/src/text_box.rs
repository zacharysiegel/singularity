use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use raylib::math::Rectangle;
use raylib::RaylibHandle;
use shared::defaults::DEFAULT_RECTANGLE;
use shared::map::RenderCoord;

#[derive(Debug)]
pub struct TextBox {
    pub rectangle: Rectangle,
    pub text: String,
    pub focused: bool,
    pub hovered: bool,
}

impl ClickHandler for TextBox {
    fn click(&mut self, _rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
        if self.rectangle.check_collision_point_rec(mouse_position) {
            self.focused = true;
            ClickResult::Consume
        } else {
            self.focused = false;
            ClickResult::Pass
        }
    }
}

impl HoverHandler for TextBox {
    fn hover(&mut self, _rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        if self.rectangle.check_collision_point_rec(mouse_position) {
            self.hovered = true;
            HoverResult::Consume
        } else {
            self.hovered = false;
            HoverResult::Pass
        }
    }
}

shared::default_const_impl!(TextBox);

impl TextBox {
    pub const DEFAULT: TextBox = TextBox {
        rectangle: DEFAULT_RECTANGLE,
        text: String::new(),
        focused: false,
        hovered: false,
    };

    pub fn new(rectangle: Rectangle, text: &str) -> Self {
        TextBox {
            rectangle,
            text: String::from(text),
            ..Self::DEFAULT
        }
    }

    pub fn new_empty(rectangle: Rectangle) -> Self {
        Self::new(rectangle, "")
    }
}
