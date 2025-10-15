use crate::color::{FACILITY_DESTROYED_COLOR, FACILITY_OPERATING_COLOR, FACILITY_PLACING_COLOR};
use crate::facility::{Facility, FacilityState};
use crate::map::{MapCoord, RenderCoord};
use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;

pub fn draw_facility(rl_draw: &mut RaylibDrawHandle, facility: &Facility, map_origin: &MapCoord) {
    let map_coord: MapCoord = facility.location().map_coord();
    let render_coord: RenderCoord = map_coord.render_coord(map_origin);
    let color: Color = match facility.state() {
        FacilityState::Operating => FACILITY_OPERATING_COLOR,
        FacilityState::Placing => FACILITY_PLACING_COLOR,
        FacilityState::Destroyed => FACILITY_DESTROYED_COLOR,
    };
    facility.draw(rl_draw, render_coord, color);
}
