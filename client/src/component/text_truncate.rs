use raylib::math::Vector2;
use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;

const ELLIPSIS: &str = "...";

pub fn truncate_text(
    text: &str,
    font: &WeakFont,
    font_size: f32,
    font_spacing: f32,
    max_width: f32,
) -> String {
    let full_measure: Vector2 = font.measure_text(text, font_size, font_spacing);
    if full_measure.x <= max_width {
        return text.to_string();
    }

    let ellipsis_width: f32 = font.measure_text(ELLIPSIS, font_size, font_spacing).x;
    let available_width: f32 = max_width - ellipsis_width;
    if available_width <= 0. {
        return ELLIPSIS.to_string();
    }

    let chars: Vec<char> = text.chars().collect();
    let mut truncated: String = String::new();
    for ch in &chars {
        let candidate: String = format!("{truncated}{ch}");
        let measure: f32 = font.measure_text(&candidate, font_size, font_spacing).x;
        if measure > available_width {
            break;
        }
        truncated = candidate;
    }

    format!("{truncated}{ELLIPSIS}")
}
