use crate::font::DEFAULT_FONT_SPACING;
use crate::input;
use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibHandle;
use shared::color::{DIFF_HOVER_BUTTON, TEXT_COLOR, WINDOW_BACKGROUND_COLOR};
use shared::defaults::DEFAULT_RECTANGLE;
use shared::map::RenderCoord;
use shared::math;

pub const DEFAULT_LABEL_FONT_SIZE: f32 = 18.;

#[derive(Debug, Clone)]
pub struct InnerText {
    pub text: String,
    pub font_size: f32,
}

impl InnerText {
    pub fn new(text: &str, font_size: f32) -> InnerText {
        InnerText {
            text: text.to_string(),
            font_size,
        }
    }

    pub fn from_str(text: &str) -> InnerText {
        InnerText::new(text, DEFAULT_LABEL_FONT_SIZE)
    }
}

#[derive(Debug, Clone)]
pub struct RectangularButton {
    pub rectangle: Rectangle,
    pub label: Option<InnerText>,
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
        label: None,
        on_click: input::noop_on_click,
        on_hover: input::noop_on_hover,
        hovered: false,
    };

    pub fn new(rectangle: Rectangle) -> RectangularButton {
        let mut button: RectangularButton = RectangularButton::default();
        button.rectangle = rectangle;
        button
    }

    pub fn new_with_label(label: InnerText, rectangle: Rectangle) -> RectangularButton {
        let mut button: RectangularButton = Self::new(rectangle);
        button.label = Some(label);
        button
    }

    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle) {
        self.draw_background(rl_draw);
        if let Some(label) = &self.label {
            let center: Vector2 = Vector2 {
                x: self.rectangle.x + self.rectangle.width / 2.,
                y: self.rectangle.y + self.rectangle.height / 2.,
            };
            let position: Vector2 = math::centered_text_origin(
                center,
                &label.text,
                rl_draw.get_font_default(),
                label.font_size,
                DEFAULT_FONT_SPACING,
            );
            rl_draw.draw_text_ex(
                rl_draw.get_font_default(),
                &label.text,
                position,
                label.font_size,
                DEFAULT_FONT_SPACING,
                TEXT_COLOR,
            );
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
}
