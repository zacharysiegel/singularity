use crate::color::{WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR, WINDOW_INTERIOR_BORDER_COLOR};
use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::window::error::ErrorWindow;
use crate::window::hex::HexWindow;
use crate::window::pause::PauseWindow;
use raylib::ffi::{DrawLineEx, DrawRectangleLinesEx, DrawRectangleRec};
use raylib::math::Rectangle;
use raylib::prelude::Vector2;
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
    fn draw(&self, map_origin: &MapCoord);
}

/// Lower numbers indicate higher priority in the z-buffer
#[repr(u8)]
pub enum WindowLayer {
    ErrorWindowLayer = 0,
    PauseWindowLayer = 1,
    HexWindowLayer = 2,
}

pub fn draw_background(window: &HexWindow) {
    const INNER_BORDER_GAP: f32 = 10.;

    let origin: RenderCoord = window.origin.unwrap();
    let full: Rectangle = Rectangle {
        x: origin.x,
        y: origin.y,
        width: window.dimensions().x,
        height: window.dimensions().y,
    };

    unsafe {
        DrawRectangleRec(full.into(), WINDOW_BACKGROUND_COLOR.into());
        DrawLineEx(
            Vector2 {
                x: origin.x,
                y: origin.y + INNER_BORDER_GAP,
            }
            .into(),
            Vector2 {
                x: origin.x + window.dimensions().x,
                y: origin.y + INNER_BORDER_GAP,
            }
            .into(),
            1.,
            WINDOW_INTERIOR_BORDER_COLOR.into(),
        );
        DrawLineEx(
            Vector2 {
                x: origin.x,
                y: origin.y + window.dimensions().y - INNER_BORDER_GAP,
            }
            .into(),
            Vector2 {
                x: origin.x + window.dimensions().x,
                y: origin.y + window.dimensions().y - INNER_BORDER_GAP,
            }
            .into(),
            1.,
            WINDOW_INTERIOR_BORDER_COLOR.into(),
        );
        DrawLineEx(
            Vector2 {
                x: origin.x + INNER_BORDER_GAP,
                y: origin.y,
            }
            .into(),
            Vector2 {
                x: origin.x + INNER_BORDER_GAP,
                y: origin.y + window.dimensions().y,
            }
            .into(),
            1.,
            WINDOW_INTERIOR_BORDER_COLOR.into(),
        );
        DrawLineEx(
            Vector2 {
                x: origin.x + window.dimensions().x - INNER_BORDER_GAP,
                y: origin.y,
            }
            .into(),
            Vector2 {
                x: origin.x + window.dimensions().x - INNER_BORDER_GAP,
                y: origin.y + window.dimensions().y,
            }
            .into(),
            1.,
            WINDOW_INTERIOR_BORDER_COLOR.into(),
        );
        DrawRectangleLinesEx(full.into(), 1., WINDOW_BORDER_COLOR.into());
    }
}
