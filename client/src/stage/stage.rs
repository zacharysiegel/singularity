use crate::browser::BrowserState;
use crate::game::GameState;
use crate::locked_switch::LockedSwitch;
use crate::title::TitleState;
use crate::{browser, game, title};
use raylib::drawing::RaylibDrawHandle;
use raylib::{RaylibHandle, RaylibThread};

use std::sync::{RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct StageState {
    pub title: TitleState,
    pub game: GameState,
    pub browser: BrowserState,

    switch: LockedSwitch<StageType>,
}

impl StageState {
    pub const DEFAULT: StageState = StageState {
        switch: LockedSwitch::new(StageType::Title),
        title: TitleState::DEFAULT,
        game: GameState::DEFAULT,
        browser: BrowserState::DEFAULT,
    };

    pub fn current(&'_ self) -> RwLockReadGuard<'_, StageType> {
        self.switch.current.read().unwrap()
    }

    pub fn current_mut(&'_ self) -> RwLockWriteGuard<'_, StageType> {
        self.switch.current.write().unwrap()
    }

    pub fn register_next(&self, stage_type: StageType) {
        self.switch.register_next(stage_type);
    }

    pub fn update(&self) -> (Option<StageType>, StageType) {
        self.switch.update()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StageType {
    Title,
    Game,
    Browser,
}

impl StageType {
    pub fn update(&mut self, _rl: &mut RaylibHandle) {
        match self {
            _ => {}
        }
    }

    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
        match self {
            StageType::Title => title::draw(rl_draw),
            StageType::Game => game::draw(rl_draw, rl_thread),
            StageType::Browser => browser::draw(rl_draw),
        }
    }
}
