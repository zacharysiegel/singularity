use raylib::math::{Rectangle, Vector2};
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
