use raylib::math::Vector2;
use crate::map::coordinate::RenderCoord;
use crate::window::{Window, WindowLayer};

pub struct HexWindow {}

impl Window for HexWindow {
    fn origin(&self) -> RenderCoord {
        RenderCoord::default()
    }

    fn dimensions(&self) -> Vector2 {
        Vector2::default()
    }

    fn layer(&self) -> WindowLayer {
        WindowLayer::HexWindowLayer
    }
}

