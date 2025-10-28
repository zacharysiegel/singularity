use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use shared::color::{FACILITY_DESTROYED_COLOR, FACILITY_OPERATING_COLOR, FACILITY_PLACING_COLOR};
use shared::facility::{ControlCenter, Facility, FacilityState, FacilityTrait, MetalExtractor, OilExtractor};
use shared::map::{MapCoord, RenderCoord};

pub trait FacilityDrawTrait {
    fn draw(&self, rl_draw: &mut RaylibDrawHandle, render_coord: RenderCoord) -> ();
}

pub fn draw_facility(rl_draw: &mut RaylibDrawHandle, facility: Facility, map_origin: &MapCoord) {
    let map_coord: MapCoord = facility.location().map_coord();
    let render_coord: RenderCoord = map_coord.render_coord(map_origin);

    match facility {
        Facility::ControlCenter(f) => f.draw(rl_draw, render_coord),
        Facility::MetalExtractor(f) => f.draw(rl_draw, render_coord),
        Facility::OilExtractor(f) => f.draw(rl_draw, render_coord),
    }
}

impl FacilityDrawTrait for ControlCenter {
    fn draw(&self, rl_draw: &mut RaylibDrawHandle, render_coord: RenderCoord) -> () {
        let color: Color = match self.state() {
            FacilityState::Operating => FACILITY_OPERATING_COLOR,
            FacilityState::Placing => FACILITY_PLACING_COLOR,
            FacilityState::Destroyed => FACILITY_DESTROYED_COLOR,
        };
        rl_draw.draw_text("CC", render_coord.x as i32 - 10, render_coord.y as i32 - 10, 10, color);
    }
}

impl FacilityDrawTrait for MetalExtractor {
    fn draw(&self, rl_draw: &mut RaylibDrawHandle, render_coord: RenderCoord) -> () {
        let color: Color = match self.state() {
            FacilityState::Operating => FACILITY_OPERATING_COLOR,
            FacilityState::Placing => FACILITY_PLACING_COLOR,
            FacilityState::Destroyed => FACILITY_DESTROYED_COLOR,
        };
        rl_draw.draw_text("ME", render_coord.x as i32 - 10, render_coord.y as i32 - 10, 10, color);
    }
}

impl FacilityDrawTrait for OilExtractor {
    fn draw(&self, rl_draw: &mut RaylibDrawHandle, render_coord: RenderCoord) -> () {
        let color: Color = match self.state() {
            FacilityState::Operating => FACILITY_OPERATING_COLOR,
            FacilityState::Placing => FACILITY_PLACING_COLOR,
            FacilityState::Destroyed => FACILITY_DESTROYED_COLOR,
        };
        rl_draw.draw_text("OE", render_coord.x as i32 - 10, render_coord.y as i32 - 10, 10, color);
    }
}
