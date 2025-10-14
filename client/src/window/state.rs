use crate::state::STATE;
use crate::window::Window;
use crate::window::error::ErrorWindow;
use crate::window::hex::HexWindow;
use crate::window::pause::PauseWindow;
use std::sync::RwLock;

pub const WINDOW_LAYERS: [&'static RwLock<dyn Window>; 3] = [
    &STATE.stage.map.window.error,
    &STATE.stage.map.window.pause,
    &STATE.stage.map.window.hex,
];

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
