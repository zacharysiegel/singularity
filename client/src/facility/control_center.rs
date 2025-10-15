use crate::facility::{FacilityState, FacilityTrait};
use crate::map::{HexCoord, RenderCoord};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};

#[derive(Debug, Default, Copy, Clone)]
pub struct ControlCenter {
    pub location: HexCoord,
    pub state: FacilityState,
}

impl FacilityTrait for ControlCenter {
    fn location(&self) -> HexCoord {
        self.location
    }

    fn state(&self) -> FacilityState {
        self.state
    }

    fn draw(&self, rl_draw: &mut RaylibDrawHandle, render_coord: RenderCoord, color: Color) {
        rl_draw.draw_text("CC", render_coord.x as i32 - 10, render_coord.y as i32 - 10, 10, color);
    }
}

impl ControlCenter {
    pub const INFLUENCE_RADIUS: i8 = 4;
}
