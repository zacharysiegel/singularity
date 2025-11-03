use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};

pub fn draw(rl_draw: &mut RaylibDrawHandle) {
    rl_draw.clear_background(Color::FUCHSIA)
}
