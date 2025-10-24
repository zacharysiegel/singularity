use std::f32::consts::SQRT_2;
use crate::button::RectangularButton;
use crate::color::{
    DIFF_HOVER_BUTTON, RED, WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR, WINDOW_INTERIOR_BORDER_COLOR,
};
use crate::map::RenderCoord;
use crate::math;
use crate::math::SIN_FRAC_PI_4;
use crate::state::STATE;
use crate::window::{ErrorWindow, HexWindow, PauseWindow, Window, BUTTON_WIDTH};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibThread;
use std::sync::RwLockReadGuard;

pub const BORDER_GAP: f32 = 10.;
pub const BORDER_THICKNESS: f32 = 1.;
pub const BUTTON_INTERNAL_MARGIN: Vector2 = Vector2 { x: 11., y: 11. };

const X_VERTEX_N: usize = 8;

/// These windows are considered part of the "game" and will be blurred when an overlay window is active
pub fn draw_game_windows(rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
    let hex: RwLockReadGuard<HexWindow> = STATE.stage.game.window.hex.read().unwrap();
    hex.draw(rl_draw, rl_thread);
    drop(hex);
}

/// These windows are not considered part of the "game" and will not be blurred when an overlay window is active
pub fn draw_overlay_windows(rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
    let pause: RwLockReadGuard<PauseWindow> = STATE.stage.game.window.pause.read().unwrap();
    pause.draw(rl_draw, rl_thread);
    drop(pause);

    let error: RwLockReadGuard<ErrorWindow> = STATE.stage.game.window.error.read().unwrap();
    // error.draw(rl_draw, rl_thread);
    drop(error);
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
    let button: &RectangularButton = window.close_button();
    draw_side_button(rl_draw, button);
    draw_close_x(
        rl_draw,
        Vector2 {
            x: button.rectangle.x + button.rectangle.width / 2.,
            y: button.rectangle.y + button.rectangle.height / 2.,
        },
        (BUTTON_WIDTH / 2. - BUTTON_INTERNAL_MARGIN.x) * SQRT_2,
        4.5,
    )
}

pub fn draw_side_button(rl_draw: &mut RaylibDrawHandle, button: &RectangularButton) {
    draw_side_button_background(rl_draw, button);
    draw_side_button_border(rl_draw, button.rectangle);
    draw_side_button_accent(rl_draw, button.rectangle);
}

pub fn draw_side_button_lines(rl_draw: &mut RaylibDrawHandle, button: &RectangularButton) {
    draw_side_button_border(rl_draw, button.rectangle);
    draw_side_button_accent(rl_draw, button.rectangle);
}

fn draw_side_button_background(rl_draw: &mut RaylibDrawHandle, button: &RectangularButton) {
    let mut background_color: Color = WINDOW_BACKGROUND_COLOR.clone();
    if button.is_hovered() {
        background_color = math::color_add(&background_color, &DIFF_HOVER_BUTTON);
    }
    rl_draw.draw_rectangle_rec(button.rectangle, background_color);
}

fn draw_side_button_border(rl_draw: &mut RaylibDrawHandle, rect: Rectangle) {
    let vertices: &[Vector2; 4] = &[
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
    const ACCENT_HEIGHT: f32 = 10.0;
    let vertices: &[Vector2; 2] = &[
        Vector2 {
            x: rect.x + rect.width,
            y: rect.y + rect.height - ACCENT_HEIGHT,
        },
        Vector2 {
            x: rect.x + rect.width - ACCENT_HEIGHT,
            y: rect.y + rect.height,
        },
    ];
    rl_draw.draw_line_ex(vertices[0], vertices[1], BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);
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
