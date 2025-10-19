use crate::color::{DIFF_HOVER_HEX, DIFF_WITHIN_INFLUENCE, HEX_OUTLINE_ACCENTED_COLOR, HEX_OUTLINE_COLOR};
use crate::map::config::{HEX_COUNT_SQRT, HEX_RADIUS, HEX_ROTATION};
use crate::map::coordinate;
use crate::map::coordinate::HexCoord;
use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::map::state::{Hex, ResourceType};
use crate::player::Player;
use crate::state::STATE;
use crate::window::ErrorWindow;
use crate::window::HexWindow;
use crate::window::PauseWindow;
use crate::window::Window;
use crate::{facility, math};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use std::sync::RwLockReadGuard;

const HEX_SIDES: u8 = 6;
const HEX_OUTLINE_THICKNESS: f32 = 1.;

pub fn draw_map(rl_draw: &mut RaylibDrawHandle, map_origin: &MapCoord) {
    loop_hexes(rl_draw, map_origin, draw_hex);
    loop_hexes(rl_draw, map_origin, draw_player_influence_outlines);
}

fn loop_hexes<F>(rl_draw: &mut RaylibDrawHandle, map_origin: &MapCoord, callback: F)
where
    F: Fn(&mut RaylibDrawHandle, &MapCoord, HexCoord) -> (),
{
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
            callback(rl_draw, map_origin, hex_coord);

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

fn draw_hex(rl_draw: &mut RaylibDrawHandle, map_origin: &MapCoord, hex_coord: HexCoord) {
    let Some(hex): Option<Hex> = hex_coord.clone_map_hex() else {
        panic!("Invalid hex coord: {:?}", hex_coord);
    };
    let map_coord: MapCoord = hex_coord.map_coord();
    let render_coord: RenderCoord = map_coord.render_coord(map_origin);

    let selected_player_i: RwLockReadGuard<usize> = STATE.stage.game.player.selected.read().unwrap();
    let players: RwLockReadGuard<Vec<Player>> = STATE.stage.game.player.players.read().unwrap();
    let selected_player: &Player = &players[*selected_player_i];
    drop(selected_player_i);

    draw_hex_background(rl_draw, &hex, render_coord, selected_player);
    draw_hex_outline(rl_draw, render_coord);
}

fn draw_hex_background(rl_draw: &mut RaylibDrawHandle, hex: &Hex, render_coord: RenderCoord, selected_player: &Player) {
    let mut color: Color = hex.resource_type.color();
    let mut hovered: bool = false;

    let hovered_hex_coord: RwLockReadGuard<Option<HexCoord>> = STATE.stage.game.map.hovered_hex_coord.read().unwrap();
    if let Some(hovered_hex_coord) = *hovered_hex_coord {
        if hex.hex_coord == hovered_hex_coord {
            color = math::color_add(&color, &DIFF_HOVER_HEX);
            hovered = true;
        }
    }
    drop(hovered_hex_coord);

    let mut influenced: bool = false;
    if selected_player.within_influence(hex.hex_coord) {
        color = math::color_add(&color, &DIFF_WITHIN_INFLUENCE);
        influenced = true;
    }

    if hex.resource_type != ResourceType::None || hovered || influenced {
        rl_draw.draw_poly(render_coord, i32::from(HEX_SIDES), HEX_RADIUS, HEX_ROTATION, color);
    }
}

fn draw_hex_outline(rl_draw: &mut RaylibDrawHandle, render_coord: RenderCoord) {
    rl_draw.draw_poly_lines_ex(
        render_coord.into(),
        i32::from(HEX_SIDES),
        HEX_RADIUS,
        HEX_ROTATION,
        HEX_OUTLINE_THICKNESS,
        HEX_OUTLINE_COLOR,
    );
}

fn draw_player_influence_outlines(rl_draw: &mut RaylibDrawHandle, map_origin: &MapCoord, hex_coord: HexCoord) {
    let selected_player_i: RwLockReadGuard<usize> = STATE.stage.game.player.selected.read().unwrap();
    let players: RwLockReadGuard<Vec<Player>> = STATE.stage.game.player.players.read().unwrap();
    let selected_player: &Player = &players[*selected_player_i];
    drop(selected_player_i);

    if selected_player.within_influence(hex_coord) {
        let neighbors: [HexCoord; 6] = hex_coord.neighbors();
        for i in 0..neighbors.len() {
            let neighbor: HexCoord = neighbors[i];
            if selected_player.within_influence(neighbor) {
                continue;
            }

            let shared_vertices: Option<[MapCoord; 2]> = hex_coord.shared_vertices(neighbor);
            match shared_vertices {
                None => unreachable!(),
                Some(mut v) => {
                    let render_coords: [RenderCoord; 2] = [
                        v[0].overflow_adjusted().render_coord(map_origin),
                        v[1].overflow_adjusted().render_coord(map_origin),
                    ];
                    let distance_sq: f32 = (render_coords[0].x - render_coords[1].x).powi(2)
                        + (render_coords[0].y - render_coords[1].y).powi(2);
                    if distance_sq > HEX_RADIUS.powi(2) + 0.01 {
                        // Comparing squares to avoid sqrt
                        // If the line is longer than HEX_RADIUS, then it must be wrapping
                        continue;
                    }
                    rl_draw.draw_line_ex(
                        render_coords[0],
                        render_coords[1],
                        HEX_OUTLINE_THICKNESS,
                        HEX_OUTLINE_ACCENTED_COLOR,
                    )
                }
            }
        }
    }
}

pub fn draw_players(rl_draw: &mut RaylibDrawHandle, map_origin: &MapCoord) {
    let players: RwLockReadGuard<Vec<Player>> = STATE.stage.game.player.players.read().expect("global state poisoned");
    for player in &*players {
        for facility in player.facilities.all_facilities() {
            facility::draw_facility(rl_draw, facility, map_origin);
        }
    }
}

pub fn draw_windows(rl_draw: &mut RaylibDrawHandle) {
    let hex: RwLockReadGuard<HexWindow> = STATE.stage.game.window.hex.read().unwrap();
    hex.draw(rl_draw);
    drop(hex);

    let pause: RwLockReadGuard<PauseWindow> = STATE.stage.game.window.pause.read().unwrap();
    pause.draw(rl_draw);
    drop(pause);

    let error: RwLockReadGuard<ErrorWindow> = STATE.stage.game.window.error.read().unwrap();
    // error.draw(rl_draw);
    drop(error);
}
