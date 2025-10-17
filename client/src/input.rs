use crate::map::RenderCoord;
use crate::stage::StageType;
use crate::state::STATE;
use raylib::consts::{KeyboardKey, MouseButton};
use raylib::RaylibHandle;
use std::sync::RwLockReadGuard;

#[derive(PartialEq)]
pub enum ClickResult {
    Pass,
    Consume,
}

pub trait ClickHandler {
    /// Hook to allow an object to handle a click event.
    /// The hook should return [ClickResult::Consume] to consume the event, or
    /// [ClickResult::Pass] to allow subsequent objects to handle the same event.
    fn click(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> ClickResult;
}

#[derive(PartialEq)]
pub enum HoverResult {
    Pass,
    Consume,
}

pub trait HoverHandler {
    /// Hook to allow an object to handle a mouse hover event.
    /// The hook should return [ClickResult::Consume] to consume the event, or
    /// [ClickResult::Pass] to allow subsequent objects to handle the same event.
    fn hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult;
}

#[derive(PartialEq)]
pub enum KeyPressResult {
    Pass,
    Consume,
}

pub trait KeyPressHandler {
    /// Hook to allow an object to handle a key press event.
    /// The hook should return [ClickResult::Consume] to consume the event, or
    /// [ClickResult::Pass] to allow subsequent objects to handle the same event.
    fn key_press(&mut self, rl: &mut RaylibHandle, key: KeyboardKey) -> HoverResult;
}

pub fn handle_user_input(rl: &mut RaylibHandle) {
    let mouse_position: RenderCoord = RenderCoord(rl.get_mouse_position());

    hover(rl, mouse_position);
    if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
        click(rl, mouse_position);
    }

    if let Some(key) = rl.get_key_pressed() {
        key_press(rl, key);
    }
}

fn click(rl: &mut RaylibHandle, mouse_position: RenderCoord) {
    let current_stage: RwLockReadGuard<StageType> = STATE.stage.current.read().unwrap();
    current_stage.click(rl, mouse_position);
}

fn hover(rl: &mut RaylibHandle, mouse_position: RenderCoord) {
    let current_stage: RwLockReadGuard<StageType> = STATE.stage.current.read().unwrap();
    current_stage.hover(rl, mouse_position);
}

fn key_press(rl: &mut RaylibHandle, key: KeyboardKey) {
    let current_stage: RwLockReadGuard<StageType> = STATE.stage.current.read().unwrap();
    current_stage.key_press(rl, key);
}

pub fn noop_on_click(_rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> ClickResult {
    ClickResult::Consume
}

pub fn noop_on_hover(_rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> HoverResult {
    HoverResult::Consume
}

pub fn noop_on_key_press(_rl: &mut RaylibHandle, _key: KeyboardKey) -> KeyPressResult {
    KeyPressResult::Consume
}
