use crate::color::{RED, WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR, WINDOW_INTERIOR_BORDER_COLOR};
use crate::map::coordinate::RenderCoord;
use crate::util;
use crate::util::SIN_FRAC_PI_4;
use crate::window::Window;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};

pub const BUTTON_WIDTH: f32 = 42.;
pub const BORDER_GAP: f32 = 10.;
pub const BORDER_THICKNESS: f32 = 1.;

const X_VERTEX_N: usize = 8;

pub fn button_rectangle(window: &dyn Window, button_index: i16) -> Rectangle {
    let origin: RenderCoord = window.origin().unwrap();
    Rectangle {
        x: origin.x + window.dimensions().x - BUTTON_WIDTH - BORDER_GAP,
        y: origin.y + BORDER_GAP + (f32::from(button_index) * BUTTON_WIDTH),
        width: BUTTON_WIDTH,
        height: BUTTON_WIDTH,
    }
}

pub fn draw_window_base(rl_draw: &mut RaylibDrawHandle, window: &dyn Window) {
    draw_background(rl_draw, window);
    draw_close_button(rl_draw, window);
}

fn draw_background(rl_draw: &mut RaylibDrawHandle, window: &dyn Window) {
    let origin: RenderCoord = window.origin().unwrap();
    let full: Rectangle = Rectangle {
        x: origin.x,
        y: origin.y,
        width: window.dimensions().x,
        height: window.dimensions().y,
    };

    rl_draw.draw_rectangle_rec(full, WINDOW_BACKGROUND_COLOR);
    rl_draw.draw_line_ex(
        Vector2 {
            x: origin.x,
            y: origin.y + BORDER_GAP,
        },
        Vector2 {
            x: origin.x + window.dimensions().x,
            y: origin.y + BORDER_GAP,
        },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 {
            x: origin.x,
            y: origin.y + window.dimensions().y - BORDER_GAP,
        },
        Vector2 {
            x: origin.x + window.dimensions().x,
            y: origin.y + window.dimensions().y - BORDER_GAP,
        },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 {
            x: origin.x + BORDER_GAP,
            y: origin.y,
        },
        Vector2 {
            x: origin.x + BORDER_GAP,
            y: origin.y + window.dimensions().y,
        },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 {
            x: origin.x + window.dimensions().x - BORDER_GAP,
            y: origin.y,
        },
        Vector2 {
            x: origin.x + window.dimensions().x - BORDER_GAP,
            y: origin.y + window.dimensions().y,
        },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_rectangle_lines_ex(full, BORDER_THICKNESS, WINDOW_BORDER_COLOR);
}

fn draw_close_button(rl_draw: &mut RaylibDrawHandle, window: &dyn Window) {
    let rect: Rectangle = button_rectangle(window, 0);
    let vertices = &[
        Vector2 { x: rect.x, y: rect.y },
        Vector2 {
            x: rect.x,
            y: rect.y + BUTTON_WIDTH,
        },
        Vector2 {
            x: rect.x + BUTTON_WIDTH,
            y: rect.y + BUTTON_WIDTH,
        },
        Vector2 {
            x: rect.x + BUTTON_WIDTH,
            y: rect.y,
        },
    ];

    draw_close_background(rl_draw, rect);

    rl_draw.draw_line_ex(vertices[0], vertices[1], BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);
    rl_draw.draw_line_ex(vertices[1], vertices[2], BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);
    rl_draw.draw_line_ex(vertices[2], vertices[3], BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);
    rl_draw.draw_line_ex(vertices[3], vertices[0], BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);

    draw_close_x(
        rl_draw,
        Vector2 {
            x: rect.x + BUTTON_WIDTH / 2.,
            y: rect.y + BUTTON_WIDTH / 2.,
        },
        14.,
        4.5,
    )
}

fn draw_close_background(rl_draw: &mut RaylibDrawHandle, rect: Rectangle) {
    let mut background_color: Color = WINDOW_BACKGROUND_COLOR.clone();
    if util::rectangle_contains(rect, rl_draw.get_mouse_position()) {
        background_color.r += 0x10;
        background_color.g += 0x10;
        background_color.b += 0x10;
    }
    rl_draw.draw_rectangle_rec(rect, background_color);
}

fn draw_close_x(rl_draw: &mut RaylibDrawHandle, center: Vector2, radius: f32, width: f32) {
    let a: [Vector2; X_VERTEX_N] = create_close_x_segment(center, radius, width, false);
    let b: [Vector2; X_VERTEX_N] = create_close_x_segment(center, radius, width, true);

    rl_draw.draw_triangle_fan(&a, RED);
    rl_draw.draw_triangle_fan(&b, RED);
}

fn create_close_x_segment(center: Vector2, radius: f32, width: f32, reflect: bool) -> [Vector2; X_VERTEX_N] {
    let r_sin_frac_pi_4: f32 = radius * *SIN_FRAC_PI_4 as f32;
    let point_hypotenuse: f32 = width / 2. / *SIN_FRAC_PI_4 as f32;

    // Generate all vertices centered at (0, 0), then transform
    let mut vertices: [Vector2; X_VERTEX_N] = [
        Vector2 { x: 0., y: 0. },
        Vector2 {
            x: -r_sin_frac_pi_4,
            y: -r_sin_frac_pi_4,
        },
        Vector2 {
            x: -r_sin_frac_pi_4 + point_hypotenuse,
            y: -r_sin_frac_pi_4,
        },
        Vector2 {
            x: r_sin_frac_pi_4,
            y: r_sin_frac_pi_4 - point_hypotenuse,
        },
        Vector2 {
            x: r_sin_frac_pi_4,
            y: r_sin_frac_pi_4,
        },
        Vector2 {
            x: r_sin_frac_pi_4 - point_hypotenuse,
            y: r_sin_frac_pi_4,
        },
        Vector2 {
            x: -r_sin_frac_pi_4,
            y: -r_sin_frac_pi_4 + point_hypotenuse,
        },
        Vector2 {
            x: -r_sin_frac_pi_4,
            y: -r_sin_frac_pi_4,
        },
    ];

    if reflect {
        for vertex in &mut vertices {
            vertex.x = -vertex.x;
        }
    } else {
        vertices.reverse();
    }

    for vertex in &mut vertices {
        vertex.x = vertex.x + center.x;
        vertex.y = vertex.y + center.y;
    }

    vertices
}
