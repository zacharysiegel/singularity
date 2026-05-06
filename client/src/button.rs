use crate::input;
use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use raylib::math::Rectangle;
use raylib::RaylibHandle;
use shared::defaults::DEFAULT_RECTANGLE;
use shared::map::RenderCoord;

#[derive(Debug, Clone)]
pub struct RectangularButton {
    pub rectangle: Rectangle,
    pub text: Option<String>,
    pub on_click: fn(rl: &mut RaylibHandle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult,
    pub on_hover: fn(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult,

    hovered: bool,
}

impl ClickHandler for RectangularButton {
    fn click(&mut self, rl: &mut RaylibHandle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
        if !self.rectangle.check_collision_point_rec(press_position)
            || !self.rectangle.check_collision_point_rec(release_position)
        {
            return ClickResult::Pass;
        }
        (self.on_click)(rl, press_position, release_position)
    }
}

impl HoverHandler for RectangularButton {
    fn hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        if self.rectangle.check_collision_point_rec(mouse_position) {
            self.hovered = true;
            (self.on_hover)(rl, mouse_position)
        } else {
            self.hovered = false;
            HoverResult::Pass
        }
    }
}

shared::default_const_impl!(RectangularButton);

impl RectangularButton {
    pub const DEFAULT: RectangularButton = RectangularButton {
        rectangle: DEFAULT_RECTANGLE,
        text: None,
        on_click: input::noop_on_click,
        on_hover: input::noop_on_hover,
        hovered: false,
    };

    pub fn new(rectangle: Rectangle) -> RectangularButton {
        let mut button: RectangularButton = RectangularButton::default();
        button.rectangle = rectangle;
        button
    }

    pub fn new_with_text(text: &str, rectangle: Rectangle) -> RectangularButton {
        let mut button: RectangularButton = Self::new(rectangle);
        button.text = Some(text.to_string());
        button
    }

    pub fn is_hovered(&self) -> bool {
        self.hovered
    }
}
