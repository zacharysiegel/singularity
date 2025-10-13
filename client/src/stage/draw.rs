use std::sync::RwLockReadGuard;
use raylib::drawing::RaylibDrawHandle;
use crate::map::MapCoord;
use crate::state::STATE;
use crate::{map, title};

pub fn draw_stage_title(rl_draw: &mut RaylibDrawHandle) {
    title::draw_title(rl_draw);
}

pub fn draw_stage_map(rl_draw: &mut RaylibDrawHandle) {
    let map_origin: RwLockReadGuard<MapCoord> = STATE.stage.map.map_origin.read().expect("global state poisoned");
    map::draw_map(rl_draw, &map_origin);
    map::draw_players(rl_draw, &map_origin);
    map::draw_windows(rl_draw);
    drop(map_origin);
}
