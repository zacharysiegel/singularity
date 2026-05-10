use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use shared::color::{WINDOW_BORDER_COLOR, WINDOW_INTERIOR_BORDER_COLOR};

pub const BORDER_GAP: f32 = 10.;
pub const BORDER_THICKNESS: f32 = 2.;

pub fn draw_window_frame(rl_draw: &mut RaylibDrawHandle, rect: Rectangle, background_color: Color) {
    rl_draw.draw_rectangle_rec(rect, background_color);
    draw_interior_border(rl_draw, rect);
    rl_draw.draw_rectangle_lines_ex(rect, BORDER_THICKNESS, WINDOW_BORDER_COLOR);
}

pub fn draw_interior_border(rl_draw: &mut RaylibDrawHandle, rect: Rectangle) {
    rl_draw.draw_line_ex(
        Vector2 { x: rect.x, y: rect.y + BORDER_GAP },
        Vector2 { x: rect.x + rect.width, y: rect.y + BORDER_GAP },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 { x: rect.x, y: rect.y + rect.height - BORDER_GAP },
        Vector2 { x: rect.x + rect.width, y: rect.y + rect.height - BORDER_GAP },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 { x: rect.x + BORDER_GAP, y: rect.y },
        Vector2 { x: rect.x + BORDER_GAP, y: rect.y + rect.height },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 { x: rect.x + rect.width - BORDER_GAP, y: rect.y },
        Vector2 { x: rect.x + rect.width - BORDER_GAP, y: rect.y + rect.height },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}
