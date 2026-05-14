use crate::browser::{BrowserDomain, BrowserDomainTrait};
use crate::input::{
    ClickButton, ClickHandler, ClickResult, HoverHandler, HoverResult, KeyPressHandler, KeyPressResult,
};
use crate::stage::StageType;
use crate::state::STATE;
use raylib::ffi::KeyboardKey;
use raylib::RaylibHandle;
use shared::map::RenderCoord;
use std::sync::RwLockReadGuard;

pub struct BrowserInput;

impl ClickHandler for BrowserInput {
    fn click(&mut self, _rl: &mut RaylibHandle, _button: ClickButton, _press_position: RenderCoord, _release_position: RenderCoord) -> ClickResult {
        ClickResult::Consume
    }
}

impl HoverHandler for BrowserInput {
    fn hover(&mut self, _rl: &mut RaylibHandle, _mouse_position: RenderCoord) -> HoverResult {
        HoverResult::Consume
    }
}

impl KeyPressHandler for BrowserInput {
    fn key_press(&mut self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        if key == KeyboardKey::KEY_ESCAPE {
            STATE.stage.register_next(StageType::Title);
            return KeyPressResult::Consume;
        }

        let domain_g: RwLockReadGuard<BrowserDomain> = STATE.stage.browser.domain_switch.current.read().unwrap();
        match *domain_g {
            BrowserDomain::Former(domain) => domain.key_press(rl, key),
            BrowserDomain::Participating(domain) => domain.key_press(rl, key),
            BrowserDomain::Available(domain) => domain.key_press(rl, key),
        }
    }
}
