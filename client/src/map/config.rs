use raylib::color::Color;
use shared::util;
use std::sync::LazyLock;

pub const BACKGROUND_COLOR: Color = Color {
    r: 30,
    g: 30,
    b: 30,
    a: 0xff,
};
pub const HEX_OUTLINE_COLOR: Color = Color {
    r: 0xff,
    g: 0xff,
    b: 0xff,
    a: 0x80,
};
pub const METAL_BACKGROUND_COLOR: Color = Color {
    r: 0x60,
    g: 0x50,
    b: 0x70,
    a: 0xff,
};
pub const OIL_BACKGROUND_COLOR: Color = Color {
    r: 0x58,
    g: 0x58,
    b: 0x58,
    a: 0xff,
};
// todo: refactor to signed integers
pub const HEX_COUNT_SQRT: u16 = 64;
pub const HEX_COUNT: u16 = HEX_COUNT_SQRT * HEX_COUNT_SQRT;
pub const HEX_SIDES: u8 = 6;
pub const HEX_RADIUS: u8 = 32;
// todo: refactor to f32
pub const HEX_SIDE_LENGTH: u8 = HEX_RADIUS;
// todo: same
pub const HEX_HEIGHT: LazyLock<f32> =
    LazyLock::new(|| *util::SIN_PI_DIV_3 as f32 * f32::from(HEX_RADIUS) * 2_f32);
pub const HEX_ROTATION: f32 = 30_f32;
