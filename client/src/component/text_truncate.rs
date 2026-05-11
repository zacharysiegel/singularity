use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;

use crate::component::text::Text;
use crate::component::text_wrap;

const ELLIPSIS: &str = "...";

pub fn truncate_text(
    text: &Text,
    font: &WeakFont,
    max_width: f32,
) -> String {
    let full_width: f32 = font.measure_text(&text.content, text.font_size, text.font_spacing).x;
    if full_width <= max_width {
        return text.content.clone();
    }

    let ellipsis_width: f32 = font.measure_text(ELLIPSIS, text.font_size, text.font_spacing).x;
    let available_width: f32 = max_width - ellipsis_width;
    if available_width <= 0. {
        return ELLIPSIS.to_string();
    }

    let chars: Vec<char> = text.content.chars().collect();
    let break_point: usize = text_wrap::find_break_point(&chars, font, text.font_size, text.font_spacing, available_width);
    let truncated: String = chars[..break_point].iter().collect();
    format!("{truncated}{ELLIPSIS}")
}
