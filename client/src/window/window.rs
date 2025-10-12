use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::state::STATE;
use crate::util;
use crate::window::error::ErrorWindow;
use crate::window::hex::HexWindow;
use crate::window::pause::PauseWindow;
pub use draw::*;
use raylib::math::Rectangle;
use raylib::prelude::{RaylibDrawHandle, Vector2};
use shared::error::AppError;
use std::ops::Sub;
use std::sync::RwLock;

pub const WINDOW_LAYERS: [&'static RwLock<dyn Window>; 3] =
    [&STATE.windows.error, &STATE.windows.pause, &STATE.windows.hex];

const BUTTON_WIDTH: f32 = 42.;
const BORDER_GAP: f32 = 10.;
const BORDER_THICKNESS: f32 = 1.;

/// Lower numbers indicate higher priority in the z-buffer
#[repr(u8)]
pub enum WindowLayer {
    ErrorWindowLayer = 0,
    PauseWindowLayer = 1,
    HexWindowLayer = 2,
}

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

pub trait Window: ClickHandler + HoverHandler {
    fn is_open(&self) -> bool;
    fn origin(&self) -> Option<RenderCoord>;
    fn dimensions(&self) -> Vector2;
    fn layer(&self) -> WindowLayer;
    fn draw_content(&self, rl_draw: &mut RaylibDrawHandle);
    fn handle_window_closed(&mut self);

    fn handle_window_clicked(&mut self, _offset: Vector2) -> ClickResult {
        ClickResult::Consume
    }

    fn handle_window_hovered(&mut self, _offset: Vector2) -> ClickResult {
        ClickResult::Consume
    }

    fn draw(&self, rl_draw: &mut RaylibDrawHandle)
    where
        Self: Sized,
    {
        if !self.is_open() {
            return;
        }

        draw_window_base(rl_draw, self);
        self.draw_content(rl_draw);
    }

    fn try_to_rectangle(&self) -> Result<Rectangle, AppError> {
        let origin: RenderCoord =
            self.origin().ok_or_else(|| AppError::new("Cannot create Rectangle from non-open Window"))?;
        Ok(Rectangle {
            x: origin.x,
            y: origin.y,
            width: self.dimensions().x,
            height: self.dimensions().y,
        })
    }
}

impl<T: Window> ClickHandler for T {
    fn handle_click(&mut self, mouse_position: RenderCoord) -> ClickResult {
        if !window_contains_render_coord(self, mouse_position) {
            return ClickResult::Pass;
        }
        let origin: RenderCoord = self.origin().unwrap();

        let b0_contains: bool = util::rectangle_contains(button_rectangle(self, 0), Vector2::from(mouse_position));
        if b0_contains {
            self.handle_window_closed();
            return ClickResult::Consume;
        }

        self.handle_window_clicked(mouse_position.sub(origin.0))
    }
}

impl<T: Window> HoverHandler for T {
    fn handle_hover(&mut self, mouse_position: RenderCoord) -> HoverResult {
        if window_contains_render_coord(self, mouse_position) {
            return HoverResult::Pass;
        }
        HoverResult::Consume
    }
}

fn window_contains_render_coord(window: &dyn Window, render_coord: RenderCoord) -> bool {
    if !window.is_open() {
        return false;
    }

    let rectangle: Rectangle = window.try_to_rectangle().unwrap();
    util::rectangle_contains(rectangle, Vector2::from(render_coord))
}

fn button_rectangle(window: &dyn Window, button_index: i16) -> Rectangle {
    let origin: RenderCoord = window.origin().unwrap();
    Rectangle {
        x: origin.x + window.dimensions().x - BUTTON_WIDTH - BORDER_GAP,
        y: origin.y + BORDER_GAP + (f32::from(button_index) * BUTTON_WIDTH),
        width: BUTTON_WIDTH,
        height: BUTTON_WIDTH,
    }
}

pub mod draw {
    use crate::color::{RED, WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR, WINDOW_INTERIOR_BORDER_COLOR};
    use crate::map::coordinate::RenderCoord;
    use crate::util;
    use crate::util::SIN_FRAC_PI_4;
    use crate::window::window::{button_rectangle, BORDER_GAP, BORDER_THICKNESS, BUTTON_WIDTH};
    use crate::window::Window;
    use raylib::color::Color;
    use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
    use raylib::ffi::{DrawLineEx, DrawRectangleLinesEx};
    use raylib::math::{Rectangle, Vector2};

    const POINT_N: usize = 8;

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
        let a: [Vector2; POINT_N] = create_close_x_segment(center, radius, width, false);
        let b: [Vector2; POINT_N] = create_close_x_segment(center, radius, width, true);

        rl_draw.draw_triangle_fan(&a, RED);
        rl_draw.draw_triangle_fan(&b, RED);
    }

    fn create_close_x_segment(center: Vector2, radius: f32, width: f32, reflect: bool) -> [Vector2; POINT_N] {
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

#[cfg(test)]
mod tests {
    use crate::window::{Window, WINDOW_LAYERS};
    use std::sync::RwLockReadGuard;

    #[test]
    fn validate_window_layers() {
        for i in 0..WINDOW_LAYERS.len() {
            let window: RwLockReadGuard<dyn Window> = WINDOW_LAYERS[i].read().unwrap();
            debug_assert_eq!(i as u8, window.layer() as u8);
        }
    }
}
