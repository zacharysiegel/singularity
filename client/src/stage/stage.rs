use crate::browser::BrowserState;
use crate::game::GameState;
use crate::input::{CharPressResult, ClickResult, HoverResult, KeyPressResult, ScrollResult};
use crate::locked_switch::LockedSwitch;
use crate::title::TitleState;
use crate::{browser, game, title};
use raylib::consts::KeyboardKey;
use raylib::drawing::RaylibDrawHandle;
use raylib::math::Vector2;
use raylib::{RaylibHandle, RaylibThread};
use shared::map::RenderCoord;

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

    pub fn scroll(&self, rl: &mut RaylibHandle, scroll_v: Vector2) -> ScrollResult {
        match self {
            StageType::Game => game::scroll(rl, scroll_v),
            _ => ScrollResult::Consume,
        }
    }

    pub fn click(&self, rl: &mut RaylibHandle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
        match self {
            StageType::Title => title::click(rl, press_position, release_position),
            StageType::Game => game::click(rl, press_position, release_position),
            StageType::Browser => browser::click(rl, press_position, release_position),
        }
    }

    pub fn hover(&self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        match self {
            StageType::Title => title::hover(rl, mouse_position),
            StageType::Game => game::hover(rl, mouse_position),
            StageType::Browser => browser::hover(rl, mouse_position),
        }
    }

    pub fn key_press(&self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        match self {
            StageType::Title => title::key_press(rl, key),
            StageType::Game => game::key_press(rl, key),
            StageType::Browser => browser::key_press(rl, key),
        }
    }

    pub fn char_press(&self, rl: &mut RaylibHandle, ch: char) -> CharPressResult {
        match self {
            StageType::Title => title::char_press(rl, ch),
            StageType::Game => game::char_press(rl, ch),
            _ => CharPressResult::Pass,
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
