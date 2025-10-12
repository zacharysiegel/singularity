use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use crate::map::coordinate::RenderCoord;
use crate::state::STATE;
use crate::util;
use crate::window::draw;
use crate::window::error::ErrorWindow;
use crate::window::hex::HexWindow;
use crate::window::pause::PauseWindow;
use raylib::math::Rectangle;
use raylib::prelude::{RaylibDrawHandle, Vector2};
use raylib::RaylibHandle;
use shared::error::AppError;
use std::ops::Sub;
use std::sync::{RwLock, RwLockReadGuard};

pub const WINDOW_LAYERS: [&'static RwLock<dyn Window>; 3] =
    [&STATE.windows.error, &STATE.windows.pause, &STATE.windows.hex];

pub(super) const BUTTON_WIDTH: f32 = 42.;
pub(super) const BORDER_GAP: f32 = 10.;
pub(super) const BORDER_THICKNESS: f32 = 1.;

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

pub trait Window: ClickHandler + HoverHandler {
    fn is_open(&self) -> bool;
    fn origin(&self) -> Option<RenderCoord>;
    fn dimensions(&self) -> Vector2;
    fn layer(&self) -> WindowLayer;
    fn draw_content(&self, rl_draw: &mut RaylibDrawHandle);
    fn handle_window_closed(&mut self);

    fn handle_window_clicked(&mut self, _rl: &mut RaylibHandle, _offset: Vector2) -> ClickResult {
        ClickResult::Consume
    }

    fn handle_window_hovered(&mut self, _offset: Vector2) -> ClickResult {
        ClickResult::Consume
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

impl<T: Window> ClickHandler for T {
    fn handle_click(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
        if !window_contains_render_coord(self, mouse_position) {
            return ClickResult::Pass;
        }
        let origin: RenderCoord = self.origin().unwrap();

        let b0_contains: bool = util::rectangle_contains(button_rectangle(self, 0), Vector2::from(mouse_position));
        if b0_contains {
            self.handle_window_closed();
            return ClickResult::Consume;
        }

        self.handle_window_clicked(rl, mouse_position.sub(origin.0))
    }
}

impl<T: Window> HoverHandler for T {
    fn handle_hover(&mut self, _rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        if window_contains_render_coord(self, mouse_position) {
            return HoverResult::Pass;
        }
        HoverResult::Consume
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

pub fn bounded_origin(rl: &mut RaylibHandle, origin: &mut RenderCoord, dimensions: Vector2) -> RenderCoord {
    let overflow: Vector2 = Vector2 {
        x: (origin.x + dimensions.x) - (rl.get_screen_width() as f32),
        y: (origin.y + dimensions.y) - (rl.get_screen_height() as f32),
    };

    if overflow.x > 0. {
        origin.x -= overflow.x;
    }
    if overflow.y > 0. {
        origin.y -= overflow.y;
    }
    *origin
}

fn window_contains_render_coord(window: &dyn Window, render_coord: RenderCoord) -> bool {
    if !window.is_open() {
        return false;
    }

    let rectangle: Rectangle = window.try_to_rectangle().unwrap();
    util::rectangle_contains(rectangle, Vector2::from(render_coord))
}

pub(super) fn button_rectangle(window: &dyn Window, button_index: i16) -> Rectangle {
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
    use crate::window::{Window, WINDOW_LAYERS};
    use std::sync::RwLockReadGuard;

    #[test]
    fn validate_window_layers() {
        for i in 0..WINDOW_LAYERS.len() {
            let window: RwLockReadGuard<dyn Window> = WINDOW_LAYERS[i].read().unwrap();
            debug_assert_eq!(i as u8, window.layer() as u8);
        }
    }
}
