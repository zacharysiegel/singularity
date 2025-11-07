use crate::browser::BrowserDomainTrait;
use crate::font::DEFAULT_FONT_SPACING;
use crate::input;
use crate::input::KeyPressResult;
use raylib::consts::KeyboardKey;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::RaylibHandle;
use shared::color::WHITE;

#[derive(Debug, Copy, Clone)]
pub struct Former;

impl BrowserDomainTrait for Former {
    fn draw(&self, rl_draw: &mut RaylibDrawHandle) {
        rl_draw.draw_text_ex(
            rl_draw.get_font_default(),
            "test former",
            Vector2::new(20., 20.),
            30.,
            DEFAULT_FONT_SPACING,
            WHITE,
        )
    }

    fn key_press(&self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        input::noop_on_key_press(rl, key)
    }
}
