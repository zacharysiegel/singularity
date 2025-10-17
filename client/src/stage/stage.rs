use crate::game::GameState;
use crate::input::{ClickResult, HoverResult, KeyPressResult};
use crate::map::RenderCoord;
use crate::state::STATE;
use crate::title::TitleState;
use crate::{game, title};
use raylib::consts::KeyboardKey;
use raylib::drawing::RaylibDrawHandle;
use raylib::RaylibHandle;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct StageState {
    pub current: RwLock<StageType>,
    pub next: RwLock<Option<StageType>>,
    pub title: TitleState,
    pub game: GameState,
}

impl StageState {
    pub const DEFAULT: StageState = StageState {
        current: RwLock::new(StageType::Title),
        next: RwLock::new(None),
        title: TitleState::DEFAULT,
        game: GameState::DEFAULT,
    };
}

#[derive(Debug, Copy, Clone)]
pub enum StageType {
    Title,
    Game,
}

impl StageType {
    pub fn click(&self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult {
        match self {
            StageType::Title => title::click(rl, mouse_position),
            StageType::Game => game::click(rl, mouse_position),
        }
    }

    pub fn hover(&self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        match self {
            StageType::Title => title::hover(rl, mouse_position),
            StageType::Game => game::hover(rl, mouse_position),
        }
    }

    pub fn key_press(&self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        match self {
            StageType::Game => game::key_press(rl, key),
            _ => KeyPressResult::Pass,
        }
    }

    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle) {
        match self {
            StageType::Title => super::draw::draw_stage_title(rl_draw),
            StageType::Game => super::draw::draw_stage_map(rl_draw),
        }
    }
}

pub fn register_next(stage_type: StageType) {
    let mut next: RwLockWriteGuard<Option<StageType>> = STATE.stage.next.write().unwrap();
    *next = Some(stage_type);
}

pub fn update() {
    let next: RwLockReadGuard<Option<StageType>> = STATE.stage.next.read().unwrap();
    if next.is_none() {
        return;
    }
    drop(next);

    let mut current: RwLockWriteGuard<StageType> = STATE.stage.current.write().unwrap();
    let mut next: RwLockWriteGuard<Option<StageType>> = STATE.stage.next.write().unwrap();

    *current = next.unwrap();
    *next = None;
}
