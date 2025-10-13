use raylib::color::Color;
use raylib::math::{Rectangle, Vector2};
use raylib::text::{RaylibFont, WeakFont};
use std::ops::Sub;
use std::sync::LazyLock;

pub const SIN_FRAC_PI_3: LazyLock<f64> = LazyLock::new(|| std::f64::consts::FRAC_PI_3.sin());
pub const SIN_FRAC_PI_4: LazyLock<f64> = LazyLock::new(|| std::f64::consts::FRAC_PI_4.sin());
pub const SIN_FRAC_PI_6: LazyLock<f64> = LazyLock::new(|| std::f64::consts::FRAC_PI_6.sin());
pub const TAN_FRAC_PI_6: LazyLock<f64> = LazyLock::new(|| std::f64::consts::FRAC_PI_6.tan());

pub fn rectangle_contains(rectangle: Rectangle, point: Vector2) -> bool {
    let translated: Vector2 = point.sub(Vector2::new(rectangle.x, rectangle.y));
    0. <= translated.x && translated.x < rectangle.width && 0. <= translated.y && translated.y < rectangle.height
}

pub fn color_add(a: &Color, b: &Color) -> Color {
    Color {
        r: a.r + b.r,
        g: a.g + b.g,
        b: a.b + b.b,
        a: a.a + b.a,
    }
}

/// Determine the necessary origin point where text should be rendered in order to be centered at the
/// given center point (both vertically and horizontally).
pub fn centered_text_origin(center: Vector2, text: &str, font: WeakFont, font_size: f32, spacing: f32) -> Vector2 {
    let measure: Vector2 = font.measure_text(text, font_size, spacing);
    Vector2 {
        x: center.x - measure.x / 2.,
        y: center.y - measure.y / 2.,
    }
}
