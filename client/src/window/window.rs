use crate::map::coordinate::RenderCoord;
use raylib::prelude::Vector2;
use crate::window::hex::HexWindow;
use crate::window::pause::PauseWindow;

pub struct Windows {
    hex_detail: HexWindow,
    pause: PauseWindow,
}

pub trait Window {
    fn origin(&self) -> RenderCoord;
    fn dimensions(&self) -> Vector2;
    fn layer(&self) -> WindowLayer;
}

/// Lower numbers indicate higher priority in the z-buffer
#[repr(u8)]
pub enum WindowLayer {
    ErrorWindowLayer = 0,
    PauseWindowLayer = 1,
    HexWindowLayer = 2,
}
