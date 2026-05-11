use raylib::math::Vector2;
use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;

/// Wraps text into lines that fit within `max_length` pixels. Preserves original whitespace
/// between words. Newlines in the input produce explicit line breaks. Falls back to
/// character-level wrapping for segments wider than `max_length`.
/// Does not support right-to-left text (assumes width increases monotonically with character count).
pub fn wrap_text(
    text: &str,
    font: &WeakFont,
    font_size: f32,
    font_spacing: f32,
    max_length: f32,
) -> Vec<String> {
    let mut accumulator: Vec<String> = Vec::new();
    let mut current_line: String = String::new();

    for token in split_preserving_whitespace(text) {
        if token.starts_with(|character: char| character.is_whitespace()) {
            let token_chars: Vec<char> = token.chars().collect();
            let mut index: usize = 0;

            while index < token_chars.len() {
                let character: char = token_chars[index];

                if character == '\n' {
                    accumulator.push(current_line);
                    current_line = String::new();
                } else if character == '\r' {
                    accumulator.push(current_line);
                    current_line = String::new();

                    if token_chars.get(index + 1) == Some(&'\n') {
                        index += 1; // Skip \n for CRLF style
                    }
                } else {
                    current_line.push(character);
                }

                index += 1;
            }

            continue;
        }

        // Due to kerning and spacing techniques, we cannot assume measure(a + b) = measure(a) + measure(b)
        let candidate: String = format!("{current_line}{token}");
        let measure: Vector2 = font.measure_text(&candidate, font_size, font_spacing);

        if measure.x <= max_length {
            current_line = candidate;
        } else if current_line.is_empty() {
            wrap_long_token(&mut accumulator, &token, font, font_size, font_spacing, max_length);
        } else {
            accumulator.push(current_line.trim_end().to_string());
            current_line = token;
        }
    }

    accumulator.push(current_line);
    accumulator
}

/// Splits text into alternating word and whitespace segments without discarding any characters.
/// e.g. "hello  world" -> ["hello", "  ", "world"]
fn split_preserving_whitespace(text: &str) -> Vec<String> {
    let mut segments: Vec<String> = Vec::new();
    let mut current_segment: String = String::new();
    let mut in_whitespace: bool = text.starts_with(|character: char| character.is_whitespace());

    for character in text.chars() {
        let is_whitespace: bool = character.is_whitespace();
        if is_whitespace != in_whitespace {
            segments.push(current_segment);
            current_segment = String::new();
        }

        current_segment.push(character);
        in_whitespace = is_whitespace;
    }

    if !current_segment.is_empty() {
        segments.push(current_segment);
    }

    segments
}

/// Wraps a single token which is wider than `max_length` by breaking at character boundaries.
/// Uses binary search to find break points (assumes width is monotonically increasing — not RTL-safe).
fn wrap_long_token(
    wrapped_lines: &mut Vec<String>,
    segment: &str,
    font: &WeakFont,
    font_size: f32,
    font_spacing: f32,
    max_length: f32,
) {
    let chars: Vec<char> = segment.chars().collect();
    let mut start: usize = 0;

    while start < chars.len() {
        let remaining: &[char] = &chars[start..];
        let break_point: usize = find_break_point(remaining, font, font_size, font_spacing, max_length).max(1);
        let fragment: String = remaining[..break_point].iter().collect();
        wrapped_lines.push(fragment);
        start += break_point;
    }
}

/// Binary search for the largest prefix of `chars` which fits within `max_length`.
/// Assumes width increases monotonically with character count (not RTL-safe).
pub fn find_break_point(
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
