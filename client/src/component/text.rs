use raylib::math::Vector2;
use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;

/// Wraps text into lines that fit within `max_width` pixels. Preserves original whitespace
/// between words. Newlines in the input produce explicit line breaks. Falls back to
/// character-level wrapping for segments wider than `max_width`.
pub fn wrap_text(
    text: &str,
    font: &WeakFont,
    font_size: f32,
    font_spacing: f32,
    max_width: f32,
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

        if measure.x <= max_width {
            current_line = candidate;
        } else if current_line.is_empty() {
            wrap_long_token(&mut accumulator, &token, font, font_size, font_spacing, max_width);
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

/// Wraps a single segment that is wider than `max_width` by breaking at character boundaries.
fn wrap_long_token(
    wrapped_lines: &mut Vec<String>,
    segment: &str,
    font: &WeakFont,
    font_size: f32,
    font_spacing: f32,
    max_width: f32,
) {
    let mut current_fragment: String = String::new();

    for character in segment.chars() {
        let candidate: String = format!("{current_fragment}{character}");
        let measure: Vector2 = font.measure_text(&candidate, font_size, font_spacing);
        if measure.x > max_width && !current_fragment.is_empty() {
            wrapped_lines.push(current_fragment);
            current_fragment = character.to_string();
        } else {
            current_fragment = candidate;
        }
    }

    if !current_fragment.is_empty() {
        wrapped_lines.push(current_fragment);
    }
}
