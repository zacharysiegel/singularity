use crate::facility::Facility;
use crate::map::{MapCoord, RenderCoord};
use raylib::drawing::RaylibDrawHandle;

pub fn draw_facility(rl_draw: &mut RaylibDrawHandle, facility: &Facility, map_origin: &MapCoord) {
    let map_coord: MapCoord = facility.location().map_coord();
    let render_coord: RenderCoord = map_coord.render_coord(map_origin);
    facility.draw(rl_draw, render_coord);
}
