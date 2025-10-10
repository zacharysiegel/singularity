use raylib::math::Vector2;
use crate::map::coordinate::RenderCoord;
use crate::window::{Window, WindowLayer};

pub struct ErrorWindow {}

impl Window for ErrorWindow {
    fn origin(&self) -> RenderCoord {
        todo!()
    }

    fn dimensions(&self) -> Vector2 {
        todo!()
    }

    fn layer(&self) -> WindowLayer {
        todo!()
    }
}
