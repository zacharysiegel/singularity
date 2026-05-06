use crate::stage::StageType;
use crate::state::STATE;
use raylib::consts::{KeyboardKey, MouseButton};
use raylib::math::Vector2;
use raylib::RaylibHandle;
use shared::environment::RuntimeEnvironment;
use shared::map::RenderCoord;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

static MOUSE_PRESS_POSITION: RwLock<Option<RenderCoord>> = RwLock::new(None);

#[derive(PartialEq)]
pub enum ScrollResult {
    Pass,
    Consume,
}

pub trait ScrollHandler {
    /// Hook to allow an object to handle a scroll event.
    /// The hook should return [ScrollResult::Consume] to consume the event, or
    /// [ScrollResult::Pass] to allow subsequent objects to handle the same event.
    fn scroll(&mut self, rl: &mut RaylibHandle, scroll_v: Vector2) -> ScrollResult;
}

#[derive(PartialEq)]
pub enum ClickResult {
    Pass,
    Consume,
}

pub trait ClickHandler {
    /// Hook to allow an object to handle a click event.
    /// The hook should return [ClickResult::Consume] to consume the event, or
    /// [ClickResult::Pass] to allow subsequent objects to handle the same event.
    fn click(&mut self, rl: &mut RaylibHandle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult;
}

#[derive(PartialEq)]
pub enum HoverResult {
    Pass,
    Consume,
}

pub trait HoverHandler {
    /// Hook to allow an object to handle a mouse hover event.
    /// The hook should return [HoverResult::Consume] to consume the event, or
    /// [HoverResult::Pass] to allow subsequent objects to handle the same event.
    fn hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult;
}

#[derive(PartialEq)]
pub enum KeyPressResult {
    Pass,
    Consume,
}

pub trait KeyPressHandler {
    /// Hook to allow an object to handle a key press event.
    /// The hook should return [KeyPressResult::Consume] to consume the event, or
    /// [KeyPressResult::Pass] to allow subsequent objects to handle the same event.
    fn key_press(&mut self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult;
}

#[derive(PartialEq)]
pub enum CharPressResult {
    Pass,
    Consume,
}

pub trait CharPressHandler {
    /// Hook to allow an object to handle a character input event (printable characters).
    /// The hook should return [CharPressResult::Consume] to consume the event, or
    /// [CharPressResult::Pass] to allow subsequent objects to handle the same event.
    fn char_press(&mut self, rl: &mut RaylibHandle, ch: char) -> CharPressResult;
}

pub fn handle_user_input(rl: &mut RaylibHandle) {
    let mouse_position: RenderCoord = RenderCoord(rl.get_mouse_position());
    let scroll_v: Vector2 = Vector2::from(rl.get_mouse_wheel_move_v());

    scroll(rl, scroll_v);
    hover(rl, mouse_position);

    if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
        let mut press_position: RwLockWriteGuard<Option<RenderCoord>> = MOUSE_PRESS_POSITION.write().unwrap();
        *press_position = Some(mouse_position);
    }

    if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
        let mut press_position: RwLockWriteGuard<Option<RenderCoord>> = MOUSE_PRESS_POSITION.write().unwrap();
        if let Some(press) = *press_position {
            click(rl, press, mouse_position);
        }
        *press_position = None;
    }

    if RuntimeEnvironment::default().is_debug() && rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_MIDDLE) {
        log::debug!("Position: ({}, {})", mouse_position.x, mouse_position.y);
    }

    while let Some(key) = rl.get_key_pressed() {
        key_press(rl, key);
    }

    while let Some(ch) = rl.get_char_pressed() {
        char_press(rl, ch);
    }
}

fn scroll(rl: &mut RaylibHandle, scroll_v: Vector2) {
    let current_stage: RwLockReadGuard<StageType> = STATE.stage.current();
    current_stage.scroll(rl, scroll_v);
}

fn click(rl: &mut RaylibHandle, press_position: RenderCoord, release_position: RenderCoord) {
    let current_stage: RwLockReadGuard<StageType> = STATE.stage.current();
    current_stage.click(rl, press_position, release_position);
}

fn hover(rl: &mut RaylibHandle, mouse_position: RenderCoord) {
    let current_stage: RwLockReadGuard<StageType> = STATE.stage.current();
    current_stage.hover(rl, mouse_position);
}

fn key_press(rl: &mut RaylibHandle, key: KeyboardKey) {
    let current_stage: RwLockReadGuard<StageType> = STATE.stage.current();
    current_stage.key_press(rl, key);
}

fn char_press(rl: &mut RaylibHandle, ch: char) {
    let current_stage: RwLockReadGuard<StageType> = STATE.stage.current();
    current_stage.char_press(rl, ch);
}

pub fn noop_on_click(_rl: &mut RaylibHandle, _press_position: RenderCoord, _release_position: RenderCoord) -> ClickResult {
    ClickResult::Consume
}

pub fn noop_on_hover(_rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> HoverResult {
    HoverResult::Consume
}

pub fn noop_on_key_press(_rl: &mut RaylibHandle, _key: KeyboardKey) -> KeyPressResult {
    KeyPressResult::Consume
}

pub fn noop_on_char_press(_rl: &mut RaylibHandle, _ch: char) -> CharPressResult {
    CharPressResult::Consume
}
