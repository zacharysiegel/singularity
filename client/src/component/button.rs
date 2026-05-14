use crate::component::text::Text;
use crate::input;
use crate::input::{ClickButton, ClickHandler, ClickResult, HoverHandler, HoverResult};
use raylib::RaylibHandle;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use shared::color::{DIFF_HOVER_BUTTON, WINDOW_BACKGROUND_COLOR};
use shared::defaults::DEFAULT_RECTANGLE;
use shared::map::RenderCoord;
use shared::math;

#[derive(Debug)]
pub struct RectangularButton {
    pub rectangle: Rectangle,
    pub text: Option<Text>,
    pub on_click: fn(rl: &mut RaylibHandle, button: ClickButton, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult,
    pub on_hover: fn(rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult,

    hovered: bool,
}

impl ClickHandler for RectangularButton {
    fn click(
        &mut self,
        rl: &mut RaylibHandle,
        button: ClickButton,
        press_position: RenderCoord,
        release_position: RenderCoord,
    ) -> ClickResult {
        if button != ClickButton::Left {
            return ClickResult::Pass;
        }

        if !self.rectangle.check_collision_point_rec(press_position)
            || !self.rectangle.check_collision_point_rec(release_position)
        {
            return ClickResult::Pass;
        }
        (self.on_click)(rl, button, press_position, release_position)
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

    pub fn new_with_text(rectangle: Rectangle, text: Text) -> RectangularButton {
        let mut button: RectangularButton = Self::new(rectangle);
        button.text = Some(text);
        button
    }

    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle) {
        self.draw_background(rl_draw);
        if let Some(text) = &self.text {
            self.draw_text_content(rl_draw, text);
        }
    }

    fn draw_background(&self, rl_draw: &mut RaylibDrawHandle) {
        let position: Vector2 = math::rect_origin(self.rectangle);
        let dimensions: Vector2 = math::rect_dimensions(self.rectangle);
        let mut background_color: Color = WINDOW_BACKGROUND_COLOR;
        if self.hovered {
            background_color = math::color_add(&background_color, &DIFF_HOVER_BUTTON);
        }
        rl_draw.draw_rectangle_v(position, dimensions, background_color);
    }

    fn draw_text_content(&self, rl_draw: &mut RaylibDrawHandle, text: &Text) {
        let center: Vector2 = Vector2 {
            x: self.rectangle.x + self.rectangle.width / 2.,
            y: self.rectangle.y + self.rectangle.height / 2.,
        };
        let position: Vector2 = math::centered_text_origin(
            center,
            &text.content,
            rl_draw.get_font_default(),
            text.font_size,
            text.font_spacing,
        );
        rl_draw.draw_text_ex(
            rl_draw.get_font_default(),
            &text.content,
            position,
            text.font_size,
            text.font_spacing,
            text.color,
        );
    }
}
