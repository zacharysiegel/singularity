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
    /// User/handler-set position. The visible scroll position is derived from this and the
    /// current dimensions via `scroll_offset()`. Never read this field directly for layout;
    /// it can be out of range relative to the latest dimensions.
    scroll_offset_intent: f32,
    /// When set, `scroll_offset()` reports `max_scroll()` so the view lands at the bottom
    /// using whatever content size is current. Lets callers (e.g. message handlers) request
    /// "land at the bottom" without knowing the new content height.
    pending_snap_to_bottom: bool,
}

impl VerticalScrollRegion {
    pub fn new(viewport: Rectangle, padding: f32) -> VerticalScrollRegion {
        VerticalScrollRegion {
            viewport,
            content_height: 0.,
            padding,
            scroll_offset_intent: 0.,
            pending_snap_to_bottom: false,
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

    /// The effective scroll position, derived from intent and current dimensions. Always
    /// in `[0, max_scroll()]`.
    pub fn scroll_offset(&self) -> f32 {
        if self.pending_snap_to_bottom {
            self.max_scroll()
        } else {
            self.scroll_offset_intent.clamp(0., self.max_scroll())
        }
    }

    pub fn padding(&self) -> f32 {
        self.padding
    }

    pub fn max_scroll(&self) -> f32 {
        (self.content_height + self.padding * 2. - self.viewport.height).max(0.)
    }

    pub fn is_at_bottom(&self) -> bool {
        self.pending_snap_to_bottom || self.scroll_offset_intent >= self.max_scroll()
    }

    /// Requests subsequent reads to report `max_scroll()` until the user scrolls. Defers
    /// the bottom-landing decision so callers don't need to know the post-update content
    /// height (which may depend on font/viewport state unavailable at the call site).
    pub fn request_scroll_to_bottom(&mut self) {
        self.pending_snap_to_bottom = true;
    }

    pub fn scroll_clamped(&mut self, delta: f32) {
        self.scroll_offset_intent = (self.scroll_offset() + delta).clamp(0., self.max_scroll());
        self.pending_snap_to_bottom = false;
    }

    /// Inverse of the render-side `y_offset = padding - scroll_offset()` mapping. Converts
    /// a screen y to a content-space y (origin at the first item, scroll-aware). Returns
    /// `None` if the screen y falls outside the viewport or in the leading padding gutter.
    /// Hit-testing should always go through this rather than recomputing the math at the
    /// call site, so input stays aligned with render across padding/scroll/viewport changes.
    pub fn screen_to_content_y(&self, screen_y: f32) -> Option<f32> {
        if screen_y < self.viewport.y || screen_y >= self.viewport.y + self.viewport.height {
            return None;
        }
        let content_y: f32 = screen_y - self.viewport.y + self.scroll_offset() - self.padding;
        if content_y < 0. {
            return None;
        }
        Some(content_y)
    }

    pub fn draw(
        &self,
        rl_draw: &mut RaylibDrawHandle,
        mut draw_fn: impl FnMut(RaylibScissorMode<RaylibDrawHandle>, f32),
    ) {
        let y_offset: f32 = self.padding - self.scroll_offset();
        rl_draw.draw_scissor_mode(
            self.viewport.x as i32,
            self.viewport.y as i32,
            self.viewport.width.floor() as i32 + 1, // Hacky adjustment to allow borders to render correctly with default OpenGL antialiasing/smoothing
            self.viewport.height.floor() as i32,
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
