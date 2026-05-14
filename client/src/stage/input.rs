use crate::browser::BrowserInput;
use crate::game::GameInput;
use crate::input::{
    CharPressHandler, CharPressResult, ClickButton, ClickHandler, ClickResult, HoverHandler, HoverResult,
    KeyPressHandler, KeyPressResult, ScrollHandler, ScrollResult,
};
use crate::stage::StageType;
use crate::state::STATE;
use crate::title::TitleInput;
use raylib::consts::KeyboardKey;
use raylib::math::Vector2;
use raylib::RaylibHandle;
use shared::map::RenderCoord;

pub struct StageInput;

impl StageInput {
    fn current_stage() -> StageType {
        *STATE.stage.current()
    }
}

impl ScrollHandler for StageInput {
    fn scroll(&mut self, rl: &mut RaylibHandle, scroll_v: Vector2, mouse_position: RenderCoord) -> ScrollResult {
        match Self::current_stage() {
            StageType::Title => TitleInput.scroll(rl, scroll_v, mouse_position),
            StageType::Game => GameInput.scroll(rl, scroll_v, mouse_position),
            _ => ScrollResult::Consume,
        }
    }
}

impl ClickHandler for StageInput {
    fn click(&mut self, rl: &mut RaylibHandle, button: ClickButton, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
        match Self::current_stage() {
            StageType::Title => TitleInput.click(rl, button, press_position, release_position),
            StageType::Game => GameInput.click(rl, button, press_position, release_position),
            StageType::Browser => BrowserInput.click(rl, button, press_position, release_position),
        }
    }
}

impl HoverHandler for StageInput {
    fn hover(&mut self, rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        match Self::current_stage() {
            StageType::Title => TitleInput.hover(rl, mouse_position),
            StageType::Game => GameInput.hover(rl, mouse_position),
            StageType::Browser => BrowserInput.hover(rl, mouse_position),
        }
    }
}

impl KeyPressHandler for StageInput {
    fn key_press(&mut self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        match Self::current_stage() {
            StageType::Title => TitleInput.key_press(rl, key),
            StageType::Game => GameInput.key_press(rl, key),
            StageType::Browser => BrowserInput.key_press(rl, key),
        }
    }
}

impl CharPressHandler for StageInput {
    fn char_press(&mut self, rl: &mut RaylibHandle, ch: char) -> CharPressResult {
        match Self::current_stage() {
            StageType::Title => TitleInput.char_press(rl, ch),
            StageType::Game => GameInput.char_press(rl, ch),
            _ => CharPressResult::Pass,
        }
    }
}
