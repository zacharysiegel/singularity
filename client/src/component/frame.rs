use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use shared::color::{DIFF_HOVER_BUTTON, WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR, WINDOW_INTERIOR_BORDER_COLOR};
use shared::math;

pub const BORDER_GAP: f32 = 10.;
pub const BORDER_THICKNESS: f32 = 1.;
pub const ACCENT_HEIGHT: f32 = 10.;

pub fn draw_window_frame(rl_draw: &mut RaylibDrawHandle, rect: Rectangle, background_color: Color) {
    rl_draw.draw_rectangle_rec(rect, background_color);
    draw_window_interior_border(rl_draw, rect);
    rl_draw.draw_rectangle_lines_ex(rect, BORDER_THICKNESS, WINDOW_BORDER_COLOR);
}

fn draw_window_interior_border(rl_draw: &mut RaylibDrawHandle, rect: Rectangle) {
    rl_draw.draw_line_ex(
        Vector2 {
            x: rect.x,
            y: rect.y + BORDER_GAP,
        },
        Vector2 {
            x: rect.x + rect.width,
            y: rect.y + BORDER_GAP,
        },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 {
            x: rect.x,
            y: rect.y + rect.height - BORDER_GAP,
        },
        Vector2 {
            x: rect.x + rect.width,
            y: rect.y + rect.height - BORDER_GAP,
        },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 {
            x: rect.x + BORDER_GAP,
            y: rect.y,
        },
        Vector2 {
            x: rect.x + BORDER_GAP,
            y: rect.y + rect.height,
        },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 {
            x: rect.x + rect.width - BORDER_GAP,
            y: rect.y,
        },
        Vector2 {
            x: rect.x + rect.width - BORDER_GAP,
            y: rect.y + rect.height,
        },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}

pub fn draw_side_button_frame(rl_draw: &mut RaylibDrawHandle, rect: Rectangle, hovered: bool) {
    let mut background_color: Color = WINDOW_BACKGROUND_COLOR;
    if hovered {
        background_color = math::color_add(&background_color, &DIFF_HOVER_BUTTON);
    }
    rl_draw.draw_rectangle_rec(rect, background_color);
    draw_side_button_border(rl_draw, rect);
    draw_side_button_accent(rl_draw, rect);
}

fn draw_side_button_border(rl_draw: &mut RaylibDrawHandle, rect: Rectangle) {
    let vertices: [Vector2; 4] = [
        Vector2 { x: rect.x, y: rect.y },
        Vector2 {
            x: rect.x,
            y: rect.y + rect.height,
        },
        Vector2 {
            x: rect.x + rect.width,
            y: rect.y + rect.height,
        },
        Vector2 {
            x: rect.x + rect.width,
            y: rect.y,
        },
    ];
    for i in 0..vertices.len() {
        rl_draw.draw_line_ex(
            vertices[i],
            vertices[(i + 1) % vertices.len()],
            BORDER_THICKNESS,
            WINDOW_INTERIOR_BORDER_COLOR,
        );
    }
}

fn draw_side_button_accent(rl_draw: &mut RaylibDrawHandle, rect: Rectangle) {
    rl_draw.draw_line_ex(
        Vector2 {
            x: rect.x + rect.width,
            y: rect.y + rect.height - ACCENT_HEIGHT,
        },
        Vector2 {
            x: rect.x + rect.width - ACCENT_HEIGHT,
            y: rect.y + rect.height,
        },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}

pub fn draw_side_button_accent_filled(rl_draw: &mut RaylibDrawHandle, rect: Rectangle, color: Color) {
    rl_draw.draw_triangle(
        Vector2 { x: rect.x + rect.width, y: rect.y + rect.height - ACCENT_HEIGHT },
        Vector2 { x: rect.x + rect.width - ACCENT_HEIGHT, y: rect.y + rect.height },
        Vector2 { x: rect.x + rect.width, y: rect.y + rect.height },
        color,
    );
}
