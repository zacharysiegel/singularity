use crate::input::{ScrollHandler, ScrollResult};
use raylib::drawing::{RaylibDrawHandle, RaylibScissorMode, RaylibScissorModeExt};
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibHandle;
use shared::map::RenderCoord;

const SCROLL_SPEED: f32 = 8.;

#[derive(Debug)]
pub struct VerticalScrollRegionUpdate {
    pub viewport: Option<Rectangle>,
    pub content_height: Option<f32>,
    pub padding: Option<f32>,
}

#[derive(Debug)]
pub struct VerticalScrollRegion {
    viewport: Rectangle,
    content_height: f32,
    padding: f32,
    scroll_offset: f32,
}

impl VerticalScrollRegion {
    pub fn new(viewport: Rectangle, padding: f32) -> VerticalScrollRegion {
        VerticalScrollRegion {
            viewport,
            content_height: 0.,
            padding,
            scroll_offset: 0.,
        }
    }

    pub fn update(&mut self, update: VerticalScrollRegionUpdate) {
        if let Some(viewport) = update.viewport {
            self.viewport = viewport;
        }
        if let Some(content_height) = update.content_height {
            self.content_height = content_height;
        }
        if let Some(padding) = update.padding {
            self.padding = padding;
        }
    }

    pub fn viewport(&self) -> Rectangle {
        self.viewport
    }

    pub fn scroll_offset(&self) -> f32 {
        self.scroll_offset
    }

    pub fn padding(&self) -> f32 {
        self.padding
    }

    pub fn max_scroll(&self) -> f32 {
        (self.content_height + self.padding * 2. - self.viewport.height).max(0.)
    }

    pub fn scroll_clamped(&mut self, delta: f32) {
        self.scroll_offset = (self.scroll_offset + delta).clamp(0., self.max_scroll());
    }

    pub fn draw(
        &self,
        rl_draw: &mut RaylibDrawHandle,
        mut draw_fn: impl FnMut(RaylibScissorMode<RaylibDrawHandle>, f32),
    ) {
        let y_offset: f32 = self.padding - self.scroll_offset;
        rl_draw.draw_scissor_mode(
            self.viewport.x as i32,
            self.viewport.y as i32,
            self.viewport.width as i32 + 1, // prevent scissor from clipping the right border pixel
            self.viewport.height as i32,
            |scissor_draw: RaylibScissorMode<RaylibDrawHandle>| {
                draw_fn(scissor_draw, y_offset);
            },
        );
    }
}

impl ScrollHandler for VerticalScrollRegion {
    fn scroll(&mut self, _rl: &mut RaylibHandle, scroll_v: Vector2, mouse_position: RenderCoord) -> ScrollResult {
        if !self.viewport.check_collision_point_rec(mouse_position) {
            return ScrollResult::Pass;
        }

        if scroll_v.y == 0. {
            ScrollResult::Pass
        } else {
            self.scroll_clamped(-scroll_v.y * SCROLL_SPEED);
            ScrollResult::Consume
        }
    }
}
