use crate::color::{FACILITY_DESTROYED_COLOR, FACILITY_OPERATING_COLOR, FACILITY_PLACING_COLOR};
use crate::facility::{Facility, FacilityState};
use crate::map::{MapCoord, RenderCoord};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};

pub fn draw_facility(rl_draw: &mut RaylibDrawHandle, map_origin: &MapCoord, facility: &Facility) {
    let map_coord: MapCoord = facility.location().map_coord();
    let render_coord: RenderCoord = map_coord.render_coord(map_origin);
    let color: Color = match facility.state() {
        FacilityState::Operating => FACILITY_OPERATING_COLOR,
        FacilityState::Placing => FACILITY_PLACING_COLOR,
        FacilityState::Destroyed => FACILITY_DESTROYED_COLOR,
    };

    match facility {
        Facility::ControlCenter(_) => {
            rl_draw.draw_text("CC", render_coord.x as i32 - 10, render_coord.y as i32 - 10, 10, color);
        }
        Facility::MetalExtractor(_) => {
            rl_draw.draw_text("ME", render_coord.x as i32 - 10, render_coord.y as i32 - 10, 10, color);
        }
        Facility::OilExtractor(_) => {
            rl_draw.draw_text("OE", render_coord.x as i32 - 10, render_coord.y as i32 - 10, 10, color);
        }
    }
}
