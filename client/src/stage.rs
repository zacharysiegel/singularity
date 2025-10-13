use crate::color::TEXT_COLOR;
use crate::map::{draw_map, draw_players, draw_windows, MapCoord};
use crate::state::STATE;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
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
    rl_draw.draw_text("Title", 10, 30, 30, TEXT_COLOR);
}

fn draw_stage_map(rl_draw: &mut RaylibDrawHandle) {
    let map_origin: RwLockReadGuard<MapCoord> = STATE.map.map_origin.read().expect("global state poisoned");
    draw_map(rl_draw, &map_origin);
    draw_players(rl_draw, &map_origin);
    draw_windows(rl_draw);
    drop(map_origin);
}
