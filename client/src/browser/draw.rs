use crate::browser::{BrowserDomain, BrowserDomainTrait};
use crate::state::STATE;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use shared::color::MAP_BACKGROUND_COLOR;
use std::sync::RwLockReadGuard;

pub fn draw(rl_draw: &mut RaylibDrawHandle) {
    rl_draw.clear_background(MAP_BACKGROUND_COLOR);

    let domain_g: RwLockReadGuard<BrowserDomain> = STATE.stage.browser.domain_switch.current.read().unwrap();
    match *domain_g {
        BrowserDomain::Former(domain) => domain.draw(rl_draw),
        BrowserDomain::Participating(domain) => domain.draw(rl_draw),
        BrowserDomain::Available(domain) => domain.draw(rl_draw),
    }
}
