use std::ops::{Deref, Div};
use crate::util;
use raylib::color::Color;
use std::sync::LazyLock;

pub const APPLICATION_NAME: &str = "singularity";

// todo: move to map module
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
pub const HEX_RADIUS: u8 = 32; // todo: refactor to f32
pub const HEX_SIDE_LENGTH: u8 = HEX_RADIUS; // todo: same
pub const HEX_HEIGHT: LazyLock<f32> =
    LazyLock::new(|| *util::SIN_PI_DIV_3 as f32 * f32::from(HEX_RADIUS) * 2_f32);
pub const HEX_ROTATION: f32 = 30_f32;

pub fn get_hex_count_width(pixels: f32) -> u16 {
    (pixels / *HEX_HEIGHT).ceil() as u16
}

pub fn get_hex_count_height(pixels: f32) -> u16 {
    (pixels / f32::from(HEX_RADIUS + HEX_SIDE_LENGTH / 2)).ceil() as u16
}

pub fn get_hex_width_pixels(hex_count: u16) -> f32 {
    let mut result: f32 = f32::from(hex_count) * *HEX_HEIGHT;
    if hex_count % 2 == 1 {
        result -= *HEX_HEIGHT / 2_f32;
    }
    result
}

pub fn get_hex_height_pixels(hex_count: u16) -> f32 {
    f32::from(hex_count) * f32::from(HEX_RADIUS + HEX_SIDE_LENGTH / 2)
}

pub fn get_map_width_pixels() -> f32 {
    get_hex_width_pixels(HEX_COUNT_SQRT)
}

pub fn get_map_height_pixels() -> f32 {
    get_hex_height_pixels(HEX_COUNT_SQRT)
}
