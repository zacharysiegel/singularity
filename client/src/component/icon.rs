use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use shared::color::RED;
use shared::math::SIN_FRAC_PI_4;

const X_VERTEX_N: usize = 8;

pub fn draw_close_x(rl_draw: &mut RaylibDrawHandle, center: Vector2, radius: f32, width: f32) {
    let a: [Vector2; X_VERTEX_N] = create_close_x_segment(center, radius, width, false);
    let b: [Vector2; X_VERTEX_N] = create_close_x_segment(center, radius, width, true);

    rl_draw.draw_triangle_fan(&a, RED);
    rl_draw.draw_triangle_fan(&b, RED);
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
