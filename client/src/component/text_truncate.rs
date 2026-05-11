use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;

use crate::component::text::Text;

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
    let break_point: usize = find_break_point_ltr(&chars, font, text.font_size, text.font_spacing, available_width);
    let truncated: String = chars[..break_point].iter().collect();
    format!("{truncated}{ELLIPSIS}")
}

/// Binary search for the largest prefix of `chars` which fits within `max_length`.
/// Assumes width increases monotonically with character count (not RTL-safe).
pub fn find_break_point_ltr(
    chars: &[char],
    font: &WeakFont,
    font_size: f32,
    font_spacing: f32,
    max_length: f32,
) -> usize {
    let mut low: usize = 0;
    let mut high: usize = chars.len();

    while low < high {
        // Ceiling division ensures mid > low. Without it, `low = mid` would not advance
        // when high = low + 1, causing an infinite loop.
        let mid: usize = (low + high).div_ceil(2);
        let prefix: String = chars[..mid].iter().collect();
        let measure: f32 = font.measure_text(&prefix, font_size, font_spacing).x;

        if measure <= max_length {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    low
}
