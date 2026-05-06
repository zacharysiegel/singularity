use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use shared::color::{DIFF_HOVER_BUTTON, WINDOW_BACKGROUND_COLOR};
use shared::math;

use crate::button::RectangularButton;

pub fn draw_button_background(rl_draw: &mut RaylibDrawHandle, button: &RectangularButton) {
    let position: Vector2 = math::rect_origin(button.rectangle);
    let dimensions: Vector2 = math::rect_dimensions(button.rectangle);
    let mut background_color: Color = WINDOW_BACKGROUND_COLOR;
    if button.is_hovered() {
        background_color = math::color_add(&background_color, &DIFF_HOVER_BUTTON);
    }
    rl_draw.draw_rectangle_v(position, dimensions, background_color);
}
