use crate::browser::BrowserDomain;
use crate::font::DEFAULT_FONT_SPACING;
use crate::state::STATE;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use shared::color::{MAP_BACKGROUND_COLOR, WHITE};
use std::sync::RwLockReadGuard;

pub fn draw(rl_draw: &mut RaylibDrawHandle) {
    rl_draw.clear_background(MAP_BACKGROUND_COLOR);

    let domain: RwLockReadGuard<BrowserDomain> = STATE.stage.browser.domain_switch.current.read().unwrap();
    domain.draw(rl_draw);
}

pub fn draw_former(rl_draw: &mut RaylibDrawHandle) {
    rl_draw.draw_text_ex(
        rl_draw.get_font_default(),
        "test former",
        Vector2::new(20., 20.),
        30.,
        DEFAULT_FONT_SPACING,
        WHITE,
    )
}

pub fn draw_participating(rl_draw: &mut RaylibDrawHandle) {
    rl_draw.draw_text_ex(
        rl_draw.get_font_default(),
        "test participating",
        Vector2::new(20., 20.),
        30.,
        DEFAULT_FONT_SPACING,
        WHITE,
    )
}

pub fn draw_available(rl_draw: &mut RaylibDrawHandle) {
    rl_draw.draw_text_ex(
        rl_draw.get_font_default(),
        "test available",
        Vector2::new(20., 20.),
        30.,
        DEFAULT_FONT_SPACING,
        WHITE,
    )
}
