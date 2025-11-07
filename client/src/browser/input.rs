use crate::browser::{BrowserDomain, BrowserDomainTrait};
use crate::input::KeyPressResult;
use crate::stage::StageType;
use crate::state::STATE;
use raylib::ffi::KeyboardKey;
use raylib::RaylibHandle;
use std::sync::RwLockReadGuard;

pub fn key_press(rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
    if key == KeyboardKey::KEY_ESCAPE {
        STATE.stage.switch.register_next(StageType::Title);
        return KeyPressResult::Consume;
    }

    let domain_g: RwLockReadGuard<BrowserDomain> = STATE.stage.browser.domain_switch.current.read().unwrap();
    match *domain_g {
        BrowserDomain::Former(domain) => domain.key_press(rl, key),
        BrowserDomain::Participating(domain) => domain.key_press(rl, key),
        BrowserDomain::Available(domain) => domain.key_press(rl, key),
    }
}
