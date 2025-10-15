use crate::facility::control_center::ControlCenter;
use crate::facility::metal_extractor::MetalExtractor;
use crate::facility::oil_extractor::OilExtractor;
use crate::map::{HexCoord, RenderCoord};
use raylib::drawing::RaylibDrawHandle;

#[derive(Debug, Copy, Clone)]
pub enum Facility {
    ControlCenter(ControlCenter),
    MetalExtractor(MetalExtractor),
    OilExtractor(OilExtractor),
}

impl Facility {
    pub fn location(&self) -> HexCoord {
        match self {
            Facility::ControlCenter(facility) => facility.location(),
            Facility::MetalExtractor(facility) => facility.location(),
            Facility::OilExtractor(facility) => facility.location(),
        }
    }

    pub fn state(&self) -> FacilityState {
        match self {
            Facility::ControlCenter(facility) => facility.state(),
            Facility::MetalExtractor(facility) => facility.state(),
            Facility::OilExtractor(facility) => facility.state(),
        }
    }

    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle, render_coord: RenderCoord) {
        match self {
            Facility::ControlCenter(facility) => facility.draw(rl_draw, render_coord),
            Facility::MetalExtractor(facility) => facility.draw(rl_draw, render_coord),
            Facility::OilExtractor(facility) => facility.draw(rl_draw, render_coord),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Default, Copy, Clone)]
pub enum FacilityState {
    #[default]
    Operating = 0,
    Placing,
    Destroyed,
}

pub trait FacilityTrait {
    fn location(&self) -> HexCoord;
    fn state(&self) -> FacilityState;
    fn draw(&self, rl_draw: &mut RaylibDrawHandle, render_coord: RenderCoord);
}
