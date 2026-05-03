use crate::input::{ScrollHandler, ScrollResult};
use raylib::drawing::{RaylibDrawHandle, RaylibScissorMode, RaylibScissorModeExt};
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibHandle;

const SCROLL_SPEED: f32 = 30.;

pub struct ScrollRegion {
    pub viewport: Rectangle,
    pub content_height: f32,
    pub scroll_offset: f32,
}

impl ScrollRegion {
    pub fn new(viewport: Rectangle) -> ScrollRegion {
        ScrollRegion {
            viewport,
            content_height: 0.,
            scroll_offset: 0.,
        }
    }

    pub fn max_scroll(&self) -> f32 {
        (self.content_height - self.viewport.height).max(0.)
    }

    pub fn scroll_by(&mut self, delta: f32) {
        self.scroll_offset = (self.scroll_offset + delta).clamp(0., self.max_scroll());
    }

    pub fn contains(&self, point: Vector2) -> bool {
        self.viewport.check_collision_point_rec(point)
    }

    pub fn draw(
        &self,
        rl_draw: &mut RaylibDrawHandle,
        draw_fn: impl FnMut(RaylibScissorMode<RaylibDrawHandle>, f32),
    ) {
        let y_offset: f32 = -self.scroll_offset;
        rl_draw.draw_scissor_mode(
            self.viewport.x as i32,
            self.viewport.y as i32,
            self.viewport.width as i32,
            self.viewport.height as i32,
            wrap_draw_fn(draw_fn, y_offset),
        );
    }
}

fn wrap_draw_fn(
    mut draw_fn: impl FnMut(RaylibScissorMode<RaylibDrawHandle>, f32),
    y_offset: f32,
) -> impl FnMut(RaylibScissorMode<RaylibDrawHandle>) {
    move |scissor_draw: RaylibScissorMode<RaylibDrawHandle>| {
        draw_fn(scissor_draw, y_offset);
    }
}

impl ScrollHandler for ScrollRegion {
    fn scroll(&mut self, _rl: &mut RaylibHandle, scroll_v: Vector2) -> ScrollResult {
        if scroll_v.y.abs() > 0. {
            self.scroll_by(-scroll_v.y * SCROLL_SPEED);
            ScrollResult::Consume
        } else {
            ScrollResult::Pass
        }
    }
}
