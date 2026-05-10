use std::f32;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use shared::math::SIN_FRAC_PI_4;

const X_VERTEX_N: usize = 8;

pub fn draw_close_x(rl_draw: &mut RaylibDrawHandle, bounds: Rectangle, thickness: f32, color: Color) {
    let center: Vector2 = Vector2 {
        x: bounds.x + bounds.width / 2.,
        y: bounds.y + bounds.height / 2.,
    };
    let size: f32 = bounds.width.min(bounds.height);
    let padding: f32 = size * 0.262;
    let radius: f32 = (size / 2. - padding) * f32::consts::SQRT_2;

    let a: [Vector2; X_VERTEX_N] = create_close_x_segment(center, radius, thickness, false);
    let b: [Vector2; X_VERTEX_N] = create_close_x_segment(center, radius, thickness, true);

    rl_draw.draw_triangle_fan(&a, color);
    rl_draw.draw_triangle_fan(&b, color);
}

pub fn draw_plus(rl_draw: &mut RaylibDrawHandle, bounds: Rectangle, thickness: f32, color: Color) {
    let center: Vector2 = Vector2 {
        x: bounds.x + bounds.width / 2.,
        y: bounds.y + bounds.height / 2.,
    };
    let size: f32 = bounds.width.min(bounds.height) / 4.;
    rl_draw.draw_line_ex(
        Vector2 { x: center.x - size, y: center.y },
        Vector2 { x: center.x + size, y: center.y },
        thickness,
        color,
    );
    rl_draw.draw_line_ex(
        Vector2 { x: center.x, y: center.y - size },
        Vector2 { x: center.x, y: center.y + size },
        thickness,
        color,
    );
}

pub fn draw_hamburger(rl_draw: &mut RaylibDrawHandle, bounds: Rectangle, thickness: f32, color: Color) {
    let padding_x: f32 = bounds.width * 0.28;
    let padding_y: f32 = bounds.height * 0.33;
    let line_x: f32 = bounds.x + padding_x;
    let line_width: f32 = bounds.width - padding_x * 2.;
    let line_spacing: f32 = (bounds.height - padding_y * 2.) / 2.;
    for i in 0..3 {
        let line_y: f32 = bounds.y + padding_y + (i as f32 * line_spacing);
        rl_draw.draw_line_ex(
            Vector2 { x: line_x, y: line_y },
            Vector2 { x: line_x + line_width, y: line_y },
            thickness,
            color,
        );
    }
}

fn create_close_x_segment(center: Vector2, radius: f32, width: f32, reflect: bool) -> [Vector2; X_VERTEX_N] {
    let r_sin_frac_pi_4: f32 = radius * *SIN_FRAC_PI_4 as f32;
    let point_hypotenuse: f32 = width / 2. / *SIN_FRAC_PI_4 as f32;

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
