use crate::button::RectangularButton;
use crate::map::RenderCoord;
use crate::window::state::WindowLayer;
use crate::window::Window;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::RaylibThread;

#[derive(Debug)]
pub struct ErrorWindow {
    pub origin: Option<RenderCoord>,
    pub dimensions: Vector2,
    pub close_button: RectangularButton,
}

impl Window for ErrorWindow {
    fn is_open(&self) -> bool {
        self.origin.is_some()
    }

    fn close(&mut self) {
        self.origin = None;
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

    fn close_button(&self) -> &RectangularButton {
        &self.close_button
    }

    fn close_button_mut(&mut self) -> &mut RectangularButton {
        &mut self.close_button
    }

    fn draw_content(&self, _rl_draw: &mut RaylibDrawHandle, _rl_thread: &RaylibThread) {}
}

impl ErrorWindow {
    pub const DEFAULT: ErrorWindow = ErrorWindow {
        origin: None,
        dimensions: Vector2 { x: 0., y: 0. },
        close_button: RectangularButton::DEFAULT,
    };
}
