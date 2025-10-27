use raylib::math::Vector2;
use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;

/// Determine the necessary origin point where text should be rendered in order to be centered at the
/// given center point (both vertically and horizontally).
pub fn centered_text_origin(center: Vector2, text: &str, font: WeakFont, font_size: f32, spacing: f32) -> Vector2 {
    let measure: Vector2 = font.measure_text(text, font_size, spacing);
    Vector2 {
        x: center.x - measure.x / 2.,
        y: center.y - measure.y / 2.,
    }
}
