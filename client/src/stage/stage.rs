use crate::browser::BrowserState;
use crate::game::GameState;
use crate::input::{
    CharPressHandler, CharPressResult, ClickHandler, ClickResult, HoverHandler, HoverResult,
    KeyPressHandler, KeyPressResult, ScrollHandler, ScrollResult,
};
use crate::locked_switch::LockedSwitch;
use crate::title::TitleState;
use crate::browser::BrowserInput;
use crate::game::GameInput;
use crate::title::TitleInput;
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

    pub fn scroll(&self, rl: &mut RaylibHandle, scroll_v: Vector2, mouse_position: RenderCoord) -> ScrollResult {
        match self {
            StageType::Title => TitleInput.scroll(rl, scroll_v, mouse_position),
            StageType::Game => GameInput.scroll(rl, scroll_v, mouse_position),
            _ => ScrollResult::Consume,
        }
    }

    pub fn click(&self, rl: &mut RaylibHandle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
        match self {
            StageType::Title => TitleInput.click(rl, press_position, release_position),
            StageType::Game => GameInput.click(rl, press_position, release_position),
            StageType::Browser => BrowserInput.click(rl, press_position, release_position),
        }
    }

    pub fn hover(&self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        match self {
            StageType::Title => TitleInput.hover(rl, mouse_position),
            StageType::Game => GameInput.hover(rl, mouse_position),
            StageType::Browser => BrowserInput.hover(rl, mouse_position),
        }
    }

    pub fn key_press(&self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        match self {
            StageType::Title => TitleInput.key_press(rl, key),
            StageType::Game => GameInput.key_press(rl, key),
            StageType::Browser => BrowserInput.key_press(rl, key),
        }
    }

    pub fn char_press(&self, rl: &mut RaylibHandle, ch: char) -> CharPressResult {
        match self {
            StageType::Title => TitleInput.char_press(rl, ch),
            StageType::Game => GameInput.char_press(rl, ch),
            _ => CharPressResult::Pass,
        }
    }

    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
        match self {
            StageType::Title => crate::title::draw(rl_draw),
            StageType::Game => crate::game::draw(rl_draw, rl_thread),
            StageType::Browser => crate::browser::draw(rl_draw),
        }
    }
}
