use crate::facility::{FacilityState, FacilityTrait};
use crate::map::{HexCoord, RenderCoord};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};

#[derive(Debug, Default, Copy, Clone)]
pub struct OilExtractor {
    pub location: HexCoord,
    pub state: FacilityState,
}

impl FacilityTrait for OilExtractor {
    fn location(&self) -> HexCoord {
        self.location
    }

    fn state(&self) -> FacilityState {
        self.state
    }

    fn draw(&self, rl_draw: &mut RaylibDrawHandle, render_coord: RenderCoord, color: Color) {
        rl_draw.draw_text("OE", render_coord.x as i32 - 10, render_coord.y as i32 - 10, 10, color);
    }
}
