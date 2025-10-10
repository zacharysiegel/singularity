use crate::map::coordinate::RenderCoord;
use crate::window::{Window, WindowLayer};
use raylib::math::Vector2;

#[derive(Debug)]
pub struct HexWindow {
    is_open: bool,
    origin: RenderCoord,
    dimensions: Vector2,
}

impl Window for HexWindow {
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
        WindowLayer::HexWindowLayer
    }
}

impl HexWindow {
    pub const DEFAULT: HexWindow = HexWindow {
        is_open: false,
        origin: RenderCoord {
            0: Vector2 { x: 0., y: 0. },
        },
        dimensions: Vector2 { x: 0., y: 0. },
    };
}
