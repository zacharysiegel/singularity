use crate::state::STATE;
use crate::window::error::ErrorWindow;
use crate::window::hex::HexWindow;
use crate::window::pause::PauseWindow;
use crate::window::Window;
use std::sync::{OnceLock, RwLock};

pub const WINDOW_LAYERS: [&'static RwLock<dyn Window>; 3] = [
    &STATE.stage.game.window.error,
    &STATE.stage.game.window.pause,
    &STATE.stage.game.window.hex,
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
    pub blur_shader: OnceLock<String>,
}

impl WindowState {
    pub const DEFAULT: WindowState = WindowState {
        error: RwLock::new(ErrorWindow::DEFAULT),
        pause: RwLock::new(PauseWindow::DEFAULT),
        hex: RwLock::new(HexWindow::DEFAULT),
        blur_shader: OnceLock::new(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_layers() {
        for i in 0..WINDOW_LAYERS.len() {
            assert_eq!(i, WINDOW_LAYERS[i].read().unwrap().layer() as usize);
        }
    }
}
