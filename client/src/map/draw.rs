use crate::color::{
    DIFF_HOVER_HEX, FACILITY_DESTROYED_COLOR, FACILITY_OPERATING_COLOR, FACILITY_PLACING_COLOR, HEX_OUTLINE_COLOR,
    MAP_BACKGROUND_COLOR, TEXT_COLOR,
};
use crate::map::config::{HEX_COUNT_SQRT, HEX_RADIUS, HEX_ROTATION, HEX_SIDES};
use crate::map::coordinate::HexCoord;
use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::map::state::{Hex, ResourceType};
use crate::map::coordinate;
use crate::math;
use crate::state::{Facility, FacilityState, FacilityType, Player, STATE};
use crate::window::error::ErrorWindow;
use crate::window::hex::HexWindow;
use crate::window::pause::PauseWindow;
use crate::window::Window;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::{RaylibHandle, RaylibThread};
use std::sync::RwLockReadGuard;

pub fn draw_loading_init(rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
    let mut rl_draw: RaylibDrawHandle = rl.begin_drawing(&rl_thread);
    rl_draw.clear_background(MAP_BACKGROUND_COLOR);
    rl_draw.draw_text("Loading", 16, rl_draw.get_screen_height() - 30, 20, TEXT_COLOR);
    drop(rl_draw);
}

pub fn draw_map(rl_draw: &mut RaylibDrawHandle, map_origin: &MapCoord) {
    let screen_width: i32 = rl_draw.get_screen_width();
    let screen_height: i32 = rl_draw.get_screen_height();
    let origin_hex_coord: HexCoord = map_origin.hex_coord_rect();
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

    let max_hexes_i: u16 = coordinate::get_hex_count_width(screen_width as f32);
    let max_hexes_j: u16 = coordinate::get_hex_count_height(screen_height as f32);
    for _hexes_drawn_j in 0..=(max_hexes_j + 2) {
        for _hexes_drawn_i in 0..=(max_hexes_i + 2) {
            draw_hex(rl_draw, map_origin, &hex_coord);

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

fn draw_hex(rl_draw: &mut RaylibDrawHandle, map_origin: &MapCoord, hex_coord: &HexCoord) {
    let Some(hex): Option<Hex> = hex_coord.clone_map_hex() else {
        panic!("Invalid hex coord: {:?}", hex_coord);
    };
    let map_coord: MapCoord = hex_coord.map_coord();
    let render_coord: RenderCoord = map_coord.render_coord(map_origin);

    draw_hex_background(rl_draw, &hex, &render_coord);

    rl_draw.draw_poly_lines_ex(
        render_coord.into(),
        i32::from(HEX_SIDES),
        HEX_RADIUS,
        HEX_ROTATION,
        1.,
        HEX_OUTLINE_COLOR,
    );
}

fn draw_hex_background(rl_draw: &mut RaylibDrawHandle, hex: &Hex, render_coord: &RenderCoord) {
    let mut color: Color = hex.resource_type.color();
    let mut hovered: bool = false;

    let hovered_hex_coord: RwLockReadGuard<Option<HexCoord>> = STATE.stage.map.hovered_hex_coord.read().unwrap();
    if let Some(hovered_hex_coord) = *hovered_hex_coord {
        if hex.hex_coord == hovered_hex_coord {
            color = math::color_add(&color, &DIFF_HOVER_HEX);
            hovered = true;
        }
    }

    if hex.resource_type != ResourceType::None || hovered {
        rl_draw.draw_poly(*render_coord, i32::from(HEX_SIDES), HEX_RADIUS, HEX_ROTATION, color);
    }
}

pub fn draw_players(rl_draw: &mut RaylibDrawHandle, map_origin: &MapCoord) {
    let players: RwLockReadGuard<Vec<Player>> = STATE.stage.map.player.players.read().expect("global state poisoned");
    for player in &*players {
        for facility in &player.facilities {
            draw_facility(rl_draw, map_origin, facility);
        }
    }
}

fn draw_facility(rl_draw: &mut RaylibDrawHandle, map_origin: &MapCoord, facility: &Facility) {
    let map_coord: MapCoord = facility.location.map_coord();
    let render_coord: RenderCoord = map_coord.render_coord(map_origin);
    let color: Color = match facility.facility_state {
        FacilityState::Operating => FACILITY_OPERATING_COLOR,
        FacilityState::Placing => FACILITY_PLACING_COLOR,
        FacilityState::Destroyed => FACILITY_DESTROYED_COLOR,
    };

    match facility.facility_type {
        FacilityType::ControlCenter => {
            rl_draw.draw_text("CC", render_coord.x as i32 - 10, render_coord.y as i32 - 10, 10, color);
        }
        FacilityType::MetalExtractor => {
            rl_draw.draw_text("ME", render_coord.x as i32 - 10, render_coord.y as i32 - 10, 10, color);
        }
        FacilityType::OilExtractor => {
            rl_draw.draw_text("OE", render_coord.x as i32 - 10, render_coord.y as i32 - 10, 10, color);
        }
    }
}

pub fn draw_windows(rl_draw: &mut RaylibDrawHandle) {
    let hex: RwLockReadGuard<HexWindow> = STATE.stage.map.window.hex.read().unwrap();
    hex.draw(rl_draw);
    drop(hex);

    let pause: RwLockReadGuard<PauseWindow> = STATE.stage.map.window.pause.read().unwrap();
    // pause.draw(map_origin);
    drop(pause);

    let error: RwLockReadGuard<ErrorWindow> = STATE.stage.map.window.error.read().unwrap();
    // error.draw(map_origin);
    drop(error);
}
