use crate::game::GameState;
use crate::input::{ClickHandler, ClickResult, HoverHandler, HoverResult};
use crate::map::RenderCoord;
use crate::stage::draw;
use crate::title::TitleState;
use crate::{map, title};
use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub const STAGE_TITLE: Stage = Stage {
    stage_type: StageType::Title,
};
pub const STAGE_MAP: Stage = Stage {
    stage_type: StageType::Game,
};

pub static STAGES: [RwLock<Stage>; 2] = [RwLock::new(STAGE_TITLE), RwLock::new(STAGE_MAP)];

#[derive(Debug)]
pub struct StageState {
    pub current_index: RwLock<usize>,
    pub title: TitleState,
    pub game: GameState,
}

impl StageState {
    pub const DEFAULT: StageState = StageState {
        current_index: RwLock::new(0),
        title: TitleState::DEFAULT,
        game: GameState::DEFAULT,
    };

    pub fn get_current_read<'a>(&self) -> RwLockReadGuard<'a, Stage> {
        let current_i: RwLockReadGuard<usize> = self.current_index.read().unwrap();
        let current: RwLockReadGuard<Stage> = STAGES[*current_i].read().unwrap();
        drop(current_i);
        current
    }

    pub fn get_current_write<'a>(&self) -> RwLockWriteGuard<'a, Stage> {
        let current_i: RwLockReadGuard<usize> = self.current_index.read().unwrap();
        let current: RwLockWriteGuard<Stage> = STAGES[*current_i].write().unwrap();
        drop(current_i);
        current
    }
}

#[derive(Debug)]
pub struct Stage {
    pub stage_type: StageType,
}

impl ClickHandler for Stage {
    fn handle_click(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
        match self.stage_type {
            StageType::Title => title::handle_click_title(rl, mouse_position),
            StageType::Game => map::handle_click_map(rl, mouse_position),
        }
    }
}

impl HoverHandler for Stage {
    fn handle_hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        match self.stage_type {
            StageType::Title => title::handle_hover_title(rl, mouse_position),
            StageType::Game => map::handle_hover_map(rl, mouse_position),
        }
    }
}

impl Stage {
    pub fn index(&self) -> usize {
        match self.stage_type {
            StageType::Title => 0,
            StageType::Game => 1,
        }
    }

    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle) {
        match self.stage_type {
            StageType::Title => draw::draw_stage_title(rl_draw),
            StageType::Game => draw::draw_stage_map(rl_draw),
        }
    }
}

#[derive(Debug)]
pub enum StageType {
    Title,
    Game,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index() {
        for i in 0..STAGES.len() {
            assert_eq!(i, STAGES[i].read().unwrap().index())
        }
    }
}
