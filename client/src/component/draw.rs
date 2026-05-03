use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use shared::color::{DIFF_HOVER_BUTTON, TEXT_COLOR, WINDOW_BACKGROUND_COLOR};
use shared::math;

pub fn draw_button_background(rl_draw: &mut RaylibDrawHandle, rectangle: Rectangle, hovered: bool) {
    let position: Vector2 = math::rect_origin(rectangle);
    let dimensions: Vector2 = math::rect_dimensions(rectangle);
    let mut background_color: Color = WINDOW_BACKGROUND_COLOR;
    if hovered {
        background_color = math::color_add(&background_color, &DIFF_HOVER_BUTTON);
    }
    rl_draw.draw_rectangle_v(position, dimensions, background_color);
}

pub fn draw_centered_text(
    rl_draw: &mut RaylibDrawHandle,
    text: &str,
    center: Vector2,
    font_size: f32,
    font_spacing: f32,
) {
    let position: Vector2 = math::centered_text_origin(
        center,
        text,
        rl_draw.get_font_default(),
        font_size,
        font_spacing,
    );
    rl_draw.draw_text_ex(rl_draw.get_font_default(), text, position, font_size, font_spacing, TEXT_COLOR);
}
