use crate::input::{ScrollHandler, ScrollResult};
use raylib::drawing::{RaylibDrawHandle, RaylibScissorMode, RaylibScissorModeExt};
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibHandle;
use shared::map::RenderCoord;

const SCROLL_SPEED: f32 = 8.;

#[derive(Debug)]
pub struct VerticalScrollRegion {
    pub viewport: Rectangle,
    pub content_height: f32,
    pub scroll_offset: f32,
}

impl VerticalScrollRegion {
    pub fn new(viewport: Rectangle) -> VerticalScrollRegion {
        VerticalScrollRegion {
            viewport,
            content_height: 0.,
            scroll_offset: 0.,
        }
    }

    pub fn max_scroll(&self) -> f32 {
        (self.content_height - self.viewport.height).max(0.)
    }

    pub fn scroll_clamped(&mut self, delta: f32) {
        self.scroll_offset = (self.scroll_offset + delta).clamp(0., self.max_scroll());
    }

    pub fn draw(
        &self,
        rl_draw: &mut RaylibDrawHandle,
        mut draw_fn: impl FnMut(RaylibScissorMode<RaylibDrawHandle>, f32),
    ) {
        let y_offset: f32 = -self.scroll_offset;
        rl_draw.draw_scissor_mode(
            self.viewport.x as i32,
            self.viewport.y as i32,
            self.viewport.width as i32,
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
