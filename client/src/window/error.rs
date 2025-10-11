use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::window::{Window, WindowLayer};
use raylib::math::Vector2;

#[derive(Debug)]
pub struct ErrorWindow {
    pub is_open: bool,
    pub origin: Option<RenderCoord>,
    pub dimensions: Vector2,
}

impl Window for ErrorWindow {
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
        WindowLayer::ErrorWindowLayer
    }

    fn draw(&self, _map_origin: &MapCoord) {
        todo!()
    }
}

impl ErrorWindow {
    pub const DEFAULT: ErrorWindow = ErrorWindow {
        is_open: false,
        origin: None,
        dimensions: Vector2 { x: 0., y: 0. },
    };
}
