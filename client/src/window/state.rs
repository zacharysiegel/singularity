use crate::state::STATE;
use crate::window::Window;
use crate::window::error::ErrorWindow;
use crate::window::hex::HexWindow;
use crate::window::pause::PauseWindow;
use std::sync::RwLock;

pub fn window_layers() -> [&'static RwLock<dyn Window>; 3] {
    [
        &STATE.stage.game.window.error,
        &STATE.stage.game.window.pause,
        &STATE.stage.game.window.hex,
    ]
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_window_layers() {
        let layers = window_layers();
        for i in 0..layers.len() {
            assert_eq!(i, layers[i].read().unwrap().layer() as usize);
        }
    }
}
