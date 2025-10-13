use crate::map::MapCoord;
use crate::state::STATE;
use crate::{map, title};
use raylib::drawing::RaylibDrawHandle;
use std::sync::{RwLock, RwLockReadGuard};

#[derive(Debug)]
pub struct StageState {
    pub stage_type: RwLock<StageType>,
}

impl StageState {
    pub const DEFAULT: StageState = StageState {
        stage_type: RwLock::new(StageType::Title),
    };
}

#[derive(Debug)]
pub enum StageType {
    Title,
    Map,
}

impl StageType {
    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle) {
        match self {
            StageType::Title => draw_stage_title(rl_draw),
            StageType::Map => draw_stage_map(rl_draw),
        }
    }
}

fn draw_stage_title(rl_draw: &mut RaylibDrawHandle) {
    title::draw_title(rl_draw);
}

fn draw_stage_map(rl_draw: &mut RaylibDrawHandle) {
    let map_origin: RwLockReadGuard<MapCoord> = STATE.map.map_origin.read().expect("global state poisoned");
    map::draw_map(rl_draw, &map_origin);
    map::draw_players(rl_draw, &map_origin);
    map::draw_windows(rl_draw);
    drop(map_origin);
}
