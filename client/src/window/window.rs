use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::window::error::ErrorWindow;
use crate::window::hex::HexWindow;
use crate::window::pause::PauseWindow;
use raylib::prelude::{RaylibDrawHandle, Vector2};
use std::sync::RwLock;

#[derive(Debug)]
pub struct WindowState {
    pub error: RwLock<ErrorWindow>,
    pub pause: RwLock<PauseWindow>,
    pub hex: RwLock<HexWindow>,
}

impl WindowState {
    pub const DEFAULT: WindowState = WindowState {
        error: RwLock::new(ErrorWindow::DEFAULT),
        pause: RwLock::new(PauseWindow::DEFAULT),
        hex: RwLock::new(HexWindow::DEFAULT),
    };
}

pub trait Window {
    fn is_open(&self) -> bool;
    fn origin(&self) -> Option<RenderCoord>;
    fn dimensions(&self) -> Vector2;
    fn layer(&self) -> WindowLayer;
    fn draw<'a, 'b, 'c>(&'a self, rl_draw: &'b mut RaylibDrawHandle, map_origin: &'c MapCoord);
}

/// Lower numbers indicate higher priority in the z-buffer
#[repr(u8)]
pub enum WindowLayer {
    ErrorWindowLayer = 0,
    PauseWindowLayer = 1,
    HexWindowLayer = 2,
}

pub use draw::*;
pub mod draw {
    use crate::color::{RED, WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR, WINDOW_INTERIOR_BORDER_COLOR};
    use crate::map::coordinate::RenderCoord;
    use crate::window::Window;
    use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
    use raylib::ffi::{DrawLineEx, DrawRectangleLinesEx};
    use raylib::math::{Rectangle, Vector2};
    use shared::util::SIN_FRAC_PI_4;

    const BORDER_GAP: f32 = 10.;
    const BORDER_THICKNESS: f32 = 1.;
    const POINT_N: usize = 8;

    pub fn draw_window_base<W: Window>(rl_draw: &mut RaylibDrawHandle, window: &W) {
        draw_background(rl_draw, window);
        draw_close_button(rl_draw, window);
    }

    fn draw_background<W: Window>(rl_draw: &mut RaylibDrawHandle, window: &W) {
        let origin: RenderCoord = window.origin().unwrap();
        let full: Rectangle = Rectangle {
            x: origin.x,
            y: origin.y,
            width: window.dimensions().x,
            height: window.dimensions().y,
        };

        unsafe {
            rl_draw.draw_rectangle_rec(full, WINDOW_BACKGROUND_COLOR);
            DrawLineEx(
                Vector2 {
                    x: origin.x,
                    y: origin.y + BORDER_GAP,
                }
                .into(),
                Vector2 {
                    x: origin.x + window.dimensions().x,
                    y: origin.y + BORDER_GAP,
                }
                .into(),
                BORDER_THICKNESS,
                WINDOW_INTERIOR_BORDER_COLOR.into(),
            );
            DrawLineEx(
                Vector2 {
                    x: origin.x,
                    y: origin.y + window.dimensions().y - BORDER_GAP,
                }
                .into(),
                Vector2 {
                    x: origin.x + window.dimensions().x,
                    y: origin.y + window.dimensions().y - BORDER_GAP,
                }
                .into(),
                BORDER_THICKNESS,
                WINDOW_INTERIOR_BORDER_COLOR.into(),
            );
            DrawLineEx(
                Vector2 {
                    x: origin.x + BORDER_GAP,
                    y: origin.y,
                }
                .into(),
                Vector2 {
                    x: origin.x + BORDER_GAP,
                    y: origin.y + window.dimensions().y,
                }
                .into(),
                BORDER_THICKNESS,
                WINDOW_INTERIOR_BORDER_COLOR.into(),
            );
            DrawLineEx(
                Vector2 {
                    x: origin.x + window.dimensions().x - BORDER_GAP,
                    y: origin.y,
                }
                .into(),
                Vector2 {
                    x: origin.x + window.dimensions().x - BORDER_GAP,
                    y: origin.y + window.dimensions().y,
                }
                .into(),
                BORDER_THICKNESS,
                WINDOW_INTERIOR_BORDER_COLOR.into(),
            );
            DrawRectangleLinesEx(full.into(), BORDER_THICKNESS, WINDOW_BORDER_COLOR.into());
        }
    }

    fn draw_close_button<W: Window>(rl_draw: &mut RaylibDrawHandle, window: &W) {
        const BUTTON_WIDTH: f32 = 42.;

        let origin: RenderCoord = window.origin().unwrap();
        let rect: Rectangle = Rectangle {
            x: origin.x + window.dimensions().x - BUTTON_WIDTH - BORDER_GAP,
            y: origin.y + BORDER_GAP,
            width: BUTTON_WIDTH,
            height: BUTTON_WIDTH,
        };
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

        rl_draw.draw_rectangle_rec(rect, WINDOW_BACKGROUND_COLOR);
        rl_draw.draw_line_ex(vertices[0], vertices[1], BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);
        rl_draw.draw_line_ex(vertices[1], vertices[2], BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);
        rl_draw.draw_line_ex(vertices[2], vertices[3], BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);
        rl_draw.draw_line_ex(vertices[3], vertices[0], BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);

        draw_x(
            rl_draw,
            Vector2 {
                x: rect.x + BUTTON_WIDTH / 2.,
                y: rect.y + BUTTON_WIDTH / 2.,
            },
            14.,
            4.5,
        )
    }

    fn draw_x(rl_draw: &mut RaylibDrawHandle, center: Vector2, radius: f32, width: f32) {
        let a: [Vector2; POINT_N] = create_x_segment(center, radius, width, false);
        let b: [Vector2; POINT_N] = create_x_segment(center, radius, width, true);

        rl_draw.draw_triangle_fan(&a, RED);
        rl_draw.draw_triangle_fan(&b, RED);
    }

    fn create_x_segment(center: Vector2, radius: f32, width: f32, reflect: bool) -> [Vector2; POINT_N] {
        let r_sin_frac_pi_4: f32 = radius * *SIN_FRAC_PI_4 as f32;
        let point_hypotenuse: f32 = width / 2. / *SIN_FRAC_PI_4 as f32;

        // Generate all vertices centered at (0, 0), then transform
        let mut vertices: [Vector2; POINT_N] = [
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
}
