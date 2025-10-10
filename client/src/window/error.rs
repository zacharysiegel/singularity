use crate::map::coordinate::RenderCoord;
use crate::window::{Window, WindowLayer};
use raylib::math::Vector2;

#[derive(Debug)]
pub struct ErrorWindow {
    is_open: bool,
    origin: RenderCoord,
    dimensions: Vector2,
}

impl Window for ErrorWindow {
    fn is_open(&self) -> bool {
        self.is_open
    }

    fn origin(&self) -> RenderCoord {
        self.origin
    }

    fn dimensions(&self) -> Vector2 {
        self.dimensions
    }

    fn layer(&self) -> WindowLayer {
        WindowLayer::ErrorWindowLayer
    }
}

impl ErrorWindow {
    pub const DEFAULT: ErrorWindow = ErrorWindow {
        is_open: false,
        origin: RenderCoord {
            0: Vector2 { x: 0., y: 0. },
        },
        dimensions: Vector2 { x: 0., y: 0. },
    };
}
