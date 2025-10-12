use crate::map::coordinate::RenderCoord;
use crate::window::{Window, WindowLayer};
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;

#[derive(Debug)]
pub struct PauseWindow {
    pub is_open: bool,
    pub origin: Option<RenderCoord>,
    pub dimensions: Vector2,
}

impl Window for PauseWindow {
    fn is_open(&self) -> bool {
        self.is_open
    }

    fn origin(&self) -> Option<RenderCoord> {
        self.origin
    }

    fn dimensions(&self) -> Vector2 {
        self.dimensions
    }

    fn layer(&self) -> WindowLayer {
        WindowLayer::PauseWindowLayer
    }

    fn draw_content(&self, _rl_draw: &mut RaylibDrawHandle) {
        todo!()
    }

    fn handle_window_closed(&mut self) {
        todo!()
    }
}

impl PauseWindow {
    pub const DEFAULT: PauseWindow = PauseWindow {
        is_open: false,
        origin: None,
        dimensions: Vector2 { x: 0., y: 0. },
    };
}
