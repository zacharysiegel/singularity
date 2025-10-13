use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use crate::map::RenderCoord;
use crate::stage::{draw, input};
use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;
use std::sync::RwLock;

pub const STAGE_TITLE: Stage = Stage {
    stage_type: StageType::Title,
};
pub const STAGE_MAP: Stage = Stage {
    stage_type: StageType::Map,
};

#[derive(Debug)]
pub struct StageState {
    pub current: RwLock<Stage>,
}

impl StageState {
    pub const DEFAULT: StageState = StageState {
        current: RwLock::new(STAGE_TITLE),
    };
}

#[derive(Debug)]
pub struct Stage {
    pub stage_type: StageType,
}

impl ClickHandler for Stage {
    fn handle_click(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
        match self.stage_type {
            StageType::Title => input::handle_click_title(rl, mouse_position),
            StageType::Map => input::handle_click_map(rl, mouse_position),
        }
    }
}

impl HoverHandler for Stage {
    fn handle_hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        match self.stage_type {
            StageType::Title => input::handle_hover_title(rl, mouse_position),
            StageType::Map => input::handle_hover_map(rl, mouse_position),
        }
    }
}

impl Stage {
    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle) {
        match self.stage_type {
            StageType::Title => draw::draw_stage_title(rl_draw),
            StageType::Map => draw::draw_stage_map(rl_draw),
        }
    }
}

#[derive(Debug)]
pub enum StageType {
    Title,
    Map,
}
