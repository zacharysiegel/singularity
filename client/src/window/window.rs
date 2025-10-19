use crate::button::RectangularButton;
use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult, KeyPressHandler, ScrollHandler};
use crate::map::RenderCoord;
use crate::window::draw;
use crate::window::draw::BORDER_GAP;
use crate::window::state::{WindowLayer, WINDOW_LAYERS};
use raylib::math::Rectangle;
use raylib::prelude::{RaylibDrawHandle, Vector2};
use raylib::RaylibHandle;
use shared::error::AppError;
use std::sync::RwLockReadGuard;

const BUTTON_WIDTH: f32 = 42.;

pub trait Window: ScrollHandler + ClickHandler + HoverHandler + KeyPressHandler {
    fn is_open(&self) -> bool;
    fn close(&mut self);
    fn origin(&self) -> Option<RenderCoord>;
    fn dimensions(&self) -> Vector2;
    fn layer(&self) -> WindowLayer;
    fn close_button(&self) -> &RectangularButton;
    fn close_button_mut(&mut self) -> &mut RectangularButton;
    fn draw_content(&self, rl_draw: &mut RaylibDrawHandle);

    #[allow(unused)]
    fn handle_window_clicked(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
        ClickResult::Consume
    }

    #[allow(unused)]
    fn handle_window_hovered(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        HoverResult::Consume
    }

    fn draw(&self, rl_draw: &mut RaylibDrawHandle)
    where
        Self: Sized,
    {
        if !self.is_open() {
            return;
        }

        draw::draw_window_base(rl_draw, self);
        self.draw_content(rl_draw);
    }

    fn try_to_rectangle(&self) -> Result<Rectangle, AppError> {
        let origin: RenderCoord =
            self.origin().ok_or_else(|| AppError::new("Cannot create Rectangle from non-open Window"))?;
        Ok(Rectangle {
            x: origin.x,
            y: origin.y,
            width: self.dimensions().x,
            height: self.dimensions().y,
        })
    }
}

pub fn any_window_open() -> bool {
    for window in WINDOW_LAYERS {
        let window: RwLockReadGuard<dyn Window> = window.read().unwrap();
        if window.is_open() {
            return true;
        }
    }
    false
}

/// Generate an adjusted window origin to bound the right and bottom to the screen edges
pub fn bounded_origin(rl: &mut RaylibHandle, origin: RenderCoord, dimensions: Vector2) -> RenderCoord {
    let overflow: Vector2 = Vector2 {
        x: (origin.x + dimensions.x) - (rl.get_screen_width() as f32),
        y: (origin.y + dimensions.y) - (rl.get_screen_height() as f32),
    };

    let mut bounded = RenderCoord(Vector2 { ..origin.0 });
    if overflow.x > 0. {
        bounded.x -= overflow.x;
    }
    if overflow.y > 0. {
        bounded.y -= overflow.y;
    }
    bounded
}

pub fn side_button_rectangle(window: &dyn Window, button_index: i16) -> Rectangle {
    let origin: RenderCoord = window.origin().unwrap();
    Rectangle {
        x: origin.x + window.dimensions().x - BUTTON_WIDTH - BORDER_GAP,
        y: origin.y + BORDER_GAP + (f32::from(button_index) * BUTTON_WIDTH),
        width: BUTTON_WIDTH,
        height: BUTTON_WIDTH,
    }
}

#[cfg(test)]
mod tests {
    use crate::window::state::WINDOW_LAYERS;
    use crate::window::Window;
    use std::sync::RwLockReadGuard;

    #[test]
    fn validate_window_layers() {
        for i in 0..WINDOW_LAYERS.len() {
            let window: RwLockReadGuard<dyn Window> = WINDOW_LAYERS[i].read().unwrap();
            debug_assert_eq!(i as u8, window.layer() as u8);
        }
    }
}
