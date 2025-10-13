use crate::font::DEFAULT_FONT_SPACING;
use raylib::drawing::RaylibDrawHandle;
use raylib::ffi::GetFontDefault;
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::{RaylibFont, WeakFont};
use std::sync::LazyLock;

pub const DEBUG_TEXT: &'static str = "Debug";
pub const BUTTON_TEXT_ARRAY: [&'static str; 2] = ["Account", "Games"];
pub const BUTTON_FONT_SIZE: f32 = 18.;
pub const BUTTON_MARGIN: f32 = 8.;

const BUTTON_DIMENSIONS: LazyLock<Vector2> = LazyLock::new(|| {
    let mut max_measure: Vector2 = Vector2 {
        x: f32::NEG_INFINITY,
        y: f32::NEG_INFINITY,
    };

    for text in BUTTON_TEXT_ARRAY {
        let font: WeakFont = unsafe { WeakFont::from_raw(GetFontDefault()) };
        let measure: Vector2 = font.measure_text(text, BUTTON_FONT_SIZE, DEFAULT_FONT_SPACING);
        if measure.x > max_measure.x {
            max_measure = measure;
        }
    }

    Vector2 {
        x: max_measure.x + BUTTON_MARGIN * 2.,
        y: max_measure.y + BUTTON_MARGIN * 2.,
    }
});

pub fn debug_button_rect(rl_draw: &mut RaylibDrawHandle) -> Rectangle {
    const SCREEN_MARGIN: f32 = 20.;

    Rectangle {
        x: rl_draw.get_screen_width() as f32 - SCREEN_MARGIN - BUTTON_DIMENSIONS.x,
        y: SCREEN_MARGIN,
        width: BUTTON_DIMENSIONS.x,
        height: BUTTON_DIMENSIONS.y,
    }
}
