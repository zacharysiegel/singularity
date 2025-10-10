use crate::map::coordinate::RenderCoord;
use crate::window::error::ErrorWindow;
use crate::window::hex::HexWindow;
use crate::window::pause::PauseWindow;
use raylib::prelude::Vector2;

#[derive(Debug)]
pub struct WindowState {
    error: ErrorWindow,
    pause: PauseWindow,
    hex: HexWindow,
}

impl WindowState {
    pub const DEFAULT: WindowState = WindowState {
        error: ErrorWindow::DEFAULT,
        pause: PauseWindow::DEFAULT,
        hex: HexWindow::DEFAULT,
    };
}

pub trait Window {
    fn is_open(&self) -> bool;
    fn origin(&self) -> RenderCoord;
    fn dimensions(&self) -> Vector2;
    fn layer(&self) -> WindowLayer;
    fn toggle<F>(&mut self, visitor: F)
    where
        F: FnOnce(&mut Self) -> ();
    // todo: fn draw(&self);
}

/// Lower numbers indicate higher priority in the z-buffer
#[repr(u8)]
pub enum WindowLayer {
    ErrorWindowLayer = 0,
    PauseWindowLayer = 1,
    HexWindowLayer = 2,
}
