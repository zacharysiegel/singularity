use raylib::math::Vector2;
use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;

use crate::component::text::Text;

const ELLIPSIS: &str = "...";

pub fn truncate_text(
    text: &Text,
    font: &WeakFont,
    max_width: f32,
) -> String {
    let full_measure: Vector2 = font.measure_text(&text.content, text.font_size, text.font_spacing);
    if full_measure.x <= max_width {
        return text.content.clone();
    }

    let ellipsis_width: f32 = font.measure_text(ELLIPSIS, text.font_size, text.font_spacing).x;
    let available_width: f32 = max_width - ellipsis_width;
    if available_width <= 0. {
        return ELLIPSIS.to_string();
    }

    let chars: Vec<char> = text.content.chars().collect();
    let mut truncated: String = String::new();
    for ch in &chars {
        let candidate: String = format!("{truncated}{ch}");
        let measure: f32 = font.measure_text(&candidate, text.font_size, text.font_spacing).x;
        if measure > available_width {
            break;
        }
        truncated = candidate;
    }

    format!("{truncated}{ELLIPSIS}")
}
