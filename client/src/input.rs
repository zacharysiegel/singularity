//! Input handler traits with handle semantics: the implementor is responsible for determining
//! whether an event applies to it (e.g. hit-testing mouse position against its own bounds).
//! Return `Consume` to stop propagation to subsequent handlers, or `Pass` to allow it.
//! Contrast with `on_*` callbacks/methods where the caller has already verified relevance.

use crate::conversation::ChatPanelInput;
use crate::stage::StageInput;
use raylib::consts::{KeyboardKey, MouseButton};
use raylib::math::Vector2;
use raylib::RaylibHandle;
use shared::environment::RuntimeEnvironment;
use shared::map::RenderCoord;
use std::sync::{RwLock, RwLockWriteGuard};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub enum ClickButton {
    Left,
    Middle,
    Right,
}

impl ClickButton {
    fn raylib(self) -> MouseButton {
        match self {
            ClickButton::Left => MouseButton::MOUSE_BUTTON_LEFT,
            ClickButton::Middle => MouseButton::MOUSE_BUTTON_MIDDLE,
            ClickButton::Right => MouseButton::MOUSE_BUTTON_RIGHT,
        }
    }
}

static MOUSE_LEFT_PRESS_POSITION: RwLock<Option<RenderCoord>> = RwLock::new(None);
static MOUSE_MIDDLE_PRESS_POSITION: RwLock<Option<RenderCoord>> = RwLock::new(None);
static MOUSE_RIGHT_PRESS_POSITION: RwLock<Option<RenderCoord>> = RwLock::new(None);

fn press_position_store(button: ClickButton) -> &'static RwLock<Option<RenderCoord>> {
    match button {
        ClickButton::Left => &MOUSE_LEFT_PRESS_POSITION,
        ClickButton::Middle => &MOUSE_MIDDLE_PRESS_POSITION,
        ClickButton::Right => &MOUSE_RIGHT_PRESS_POSITION,
    }
}

macro_rules! return_if_consumed {
    ($result:ident, $expression:expr) => {
        if let $result::Consume = $expression {
            return;
        }
    };
}

#[derive(PartialEq)]
pub enum ScrollResult {
    Pass,
    Consume,
}

/// Handle semantics: the implementor must determine whether the scroll event applies to it
/// (e.g. by checking if `mouse_position` is within its bounds).
pub trait ScrollHandler {
    fn scroll(&mut self, rl: &mut RaylibHandle, scroll_v: Vector2, mouse_position: RenderCoord) -> ScrollResult;
}

#[derive(PartialEq)]
pub enum ClickResult {
    Pass,
    Consume,
}

/// Handle semantics: the implementor must determine whether the click applies to it
/// (e.g. by checking if both press and release positions are within its bounds).
pub trait ClickHandler {
    fn click(&mut self, rl: &mut RaylibHandle, button: ClickButton, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult;
}

#[derive(PartialEq)]
pub enum HoverResult {
    Pass,
    Consume,
}

/// Handle semantics: the implementor must determine whether the hover applies to it
/// (e.g. by checking if `mouse_position` is within its bounds).
pub trait HoverHandler {
    fn hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult;
}

#[derive(PartialEq)]
pub enum KeyPressResult {
    Pass,
    Consume,
}

/// Handle semantics: the implementor must determine whether the key press applies to it
/// (e.g. by checking if it is focused or active).
pub trait KeyPressHandler {
    fn key_press(&mut self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult;
}

#[derive(PartialEq)]
pub enum CharPressResult {
    Pass,
    Consume,
}

/// Handle semantics: the implementor must determine whether the character input applies to it
/// (e.g. by checking if it is focused). Char events are printable characters only;
/// control keys (backspace, enter, escape) are delivered via [KeyPressHandler] instead.
pub trait CharPressHandler {
    fn char_press(&mut self, rl: &mut RaylibHandle, ch: char) -> CharPressResult;
}

pub fn handle_user_input(rl: &mut RaylibHandle) {
    let mouse_position: RenderCoord = RenderCoord(rl.get_mouse_position());
    let scroll_v: Vector2 = Vector2::from(rl.get_mouse_wheel_move_v());

    scroll(rl, scroll_v, mouse_position);
    hover(rl, mouse_position);

    for button in ClickButton::iter() {
        if rl.is_mouse_button_pressed(button.raylib()) {
            let mut press_position: RwLockWriteGuard<Option<RenderCoord>> =
                press_position_store(button).write().unwrap();
            *press_position = Some(mouse_position);
        }

        if rl.is_mouse_button_released(button.raylib()) {
            let mut press_position: RwLockWriteGuard<Option<RenderCoord>> =
                press_position_store(button).write().unwrap();
            let press: Option<RenderCoord> = *press_position;
            *press_position = None;
            drop(press_position);

            if let Some(press) = press {
                click(rl, button, press, mouse_position);

                if RuntimeEnvironment::default().is_debug() && button == ClickButton::Middle {
                    log::debug!("Position: ({}, {})", mouse_position.x, mouse_position.y);
                }
            }
        }
    }

    while let Some(key) = rl.get_key_pressed() {
        key_press(rl, key);
    }

    while let Some(ch) = rl.get_char_pressed() {
        char_press(rl, ch);
    }
}

fn scroll(rl: &mut RaylibHandle, scroll_v: Vector2, mouse_position: RenderCoord) {
    return_if_consumed!(ScrollResult, ChatPanelInput.scroll(rl, scroll_v, mouse_position));
    return_if_consumed!(ScrollResult, StageInput.scroll(rl, scroll_v, mouse_position));
}

fn click(rl: &mut RaylibHandle, button: ClickButton, press_position: RenderCoord, release_position: RenderCoord) {
    return_if_consumed!(ClickResult, ChatPanelInput.click(rl, button, press_position, release_position));
    return_if_consumed!(ClickResult, StageInput.click(rl, button, press_position, release_position));
}

fn hover(rl: &mut RaylibHandle, mouse_position: RenderCoord) {
    return_if_consumed!(HoverResult, ChatPanelInput.hover(rl, mouse_position));
    return_if_consumed!(HoverResult, StageInput.hover(rl, mouse_position));
}

fn key_press(rl: &mut RaylibHandle, key: KeyboardKey) {
    return_if_consumed!(KeyPressResult, ChatPanelInput.key_press(rl, key));
    return_if_consumed!(KeyPressResult, StageInput.key_press(rl, key));
}

fn char_press(rl: &mut RaylibHandle, ch: char) {
    return_if_consumed!(CharPressResult, ChatPanelInput.char_press(rl, ch));
    return_if_consumed!(CharPressResult, StageInput.char_press(rl, ch));
}

pub fn noop_on_click(_rl: &mut RaylibHandle, _button: ClickButton, _press_position: RenderCoord, _release_position: RenderCoord) -> ClickResult {
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
