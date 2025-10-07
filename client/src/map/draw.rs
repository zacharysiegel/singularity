use crate::map::config::{HEX_COUNT_SQRT, HEX_OUTLINE_COLOR, HEX_RADIUS, HEX_ROTATION, HEX_SIDES};
use crate::map::coordinate::{get_hex_count_height, get_hex_count_width, HexCoord};
use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::state::{Facility, FacilityState, FacilityType, Hex, ResourceType, STATE};
use raylib::color::Color;
use raylib::ffi::{DrawPoly, DrawPolyLinesEx, DrawText, GetScreenHeight, GetScreenWidth};
use std::ffi::{c_int, CString};

pub fn draw_map(map_origin: &MapCoord) {
    let screen_width: i32 = unsafe { GetScreenWidth() };
    let screen_height: i32 = unsafe { GetScreenHeight() };
    let origin_hex_coord: HexCoord = map_origin.hex_coord();
    let min_hex_coord: HexCoord = HexCoord {
        i: if origin_hex_coord.i - 1 < 0 {
            HEX_COUNT_SQRT - 1
        } else {
            origin_hex_coord.i - 1
        },
        j: if origin_hex_coord.j - 1 < 0 {
            HEX_COUNT_SQRT - 1
        } else {
            origin_hex_coord.j - 1
        },
    };
    let mut hex_coord: HexCoord = min_hex_coord;

    let max_hexes_i: u16 = get_hex_count_width(screen_width as f32);
    let max_hexes_j: u16 = get_hex_count_height(screen_height as f32);
    for _hexes_drawn_j in 0..=(max_hexes_j + 2) {
        for _hexes_drawn_i in 0..=(max_hexes_i + 2) {
            draw_map_hex(map_origin, &hex_coord);

            hex_coord.i += 1;
            if hex_coord.i >= HEX_COUNT_SQRT {
                hex_coord.i = 0;
            }
        }

        hex_coord.i = min_hex_coord.i;
        hex_coord.j += 1;
        if hex_coord.j >= HEX_COUNT_SQRT {
            hex_coord.j = 0;
        }
    }
}

fn draw_map_hex(map_origin: &MapCoord, hex_coord: &HexCoord) {
    let map_coord: MapCoord = hex_coord.map_coord();
    let render_coord: RenderCoord = map_coord.render_coord(map_origin);
    let Some(hex): Option<Hex> = hex_coord.clone_map_hex() else {
        panic!("Invalid hex coord: {:?}", hex_coord);
    };
    let color: Color = hex.resource_type.color();

    match hex.resource_type {
        ResourceType::None => {}
        _ => unsafe {
            DrawPoly(
                render_coord.into(),
                HEX_SIDES as c_int,
                f32::from(HEX_RADIUS),
                HEX_ROTATION,
                color.into(),
            );
        },
    }
    unsafe {
        DrawPolyLinesEx(
            render_coord.into(),
            HEX_SIDES as c_int,
            f32::from(HEX_RADIUS),
            HEX_ROTATION,
            1.,
            HEX_OUTLINE_COLOR.into(),
        );
    }
}

pub fn draw_players(map_origin: &MapCoord) {
    let state = STATE.read().expect("global state poisoned");
    for player in &state.players {
        for facility in &player.facilities {
            draw_facility(map_origin, facility);
        }
    }
}

fn draw_facility(map_origin: &MapCoord, facility: &Facility) {
    let map_coord: MapCoord = facility.location.map_coord();
    let render_coord: RenderCoord = map_coord.render_coord(map_origin);
    let color: Color = {
        let mut color = Color {
            r: 0xb4,
            g: 0xb4,
            b: 0xb4,
            a: 0xff,
        };
        match facility.facility_state {
            FacilityState::Operating => {}
            FacilityState::Placing => color.a = 0x80,
            FacilityState::Destroyed => color.a = 0xf0,
        }
        color
    };

    match facility.facility_type {
        FacilityType::ControlCenter => unsafe {
            let cstr = CString::new("CC").unwrap();
            DrawText(
                cstr.as_ptr(),
                render_coord.x as i32 - 10,
                render_coord.y as i32 - 10,
                10,
                color.into(),
            );
        },
        FacilityType::MetalExtractor => unsafe {
            let cstr = CString::new("ME").unwrap();
            DrawText(
                cstr.as_ptr(),
                render_coord.x as i32 - 10,
                render_coord.y as i32 - 10,
                10,
                color.into(),
            );
        },
        FacilityType::OilExtractor => unsafe {
            let cstr = CString::new("OE").unwrap();
            DrawText(
                cstr.as_ptr(),
                render_coord.x as i32 - 10,
                render_coord.y as i32 - 10,
                10,
                color.into(),
            );
        },
    }
}
