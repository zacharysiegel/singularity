use raylib::math::Vector2;
use raylib::prelude::WeakFont;
use raylib::text::RaylibFont;

pub fn wrap_text(
    text: &str,
    font: &WeakFont,
    font_size: f32,
    font_spacing: f32,
    max_width: f32,
) -> Vec<String> {
    let mut wrapped_lines: Vec<String> = Vec::new();
    let mut current_line: String = String::new();

    for word in text.split_whitespace() {
        let candidate: String = if current_line.is_empty() {
            word.to_string()
        } else {
            format!("{current_line} {word}")
        };

        let measure: Vector2 = font.measure_text(&candidate, font_size, font_spacing);
        if measure.x <= max_width {
            current_line = candidate;
        } else if current_line.is_empty() {
            wrap_long_word(&mut wrapped_lines, word, &font, font_size, font_spacing, max_width);
        } else {
            wrapped_lines.push(current_line);
            current_line = word.to_string();
        }
    }

    if !current_line.is_empty() {
        wrapped_lines.push(current_line);
    }

    if wrapped_lines.is_empty() {
        wrapped_lines.push(String::new());
    }

    wrapped_lines
}

fn wrap_long_word(
    wrapped_lines: &mut Vec<String>,
    word: &str,
    font: &WeakFont,
    font_size: f32,
    font_spacing: f32,
    max_width: f32,
) {
    let mut current_segment: String = String::new();

    for character in word.chars() {
        let candidate: String = format!("{current_segment}{character}");
        let measure: Vector2 = font.measure_text(&candidate, font_size, font_spacing);
        if measure.x > max_width && !current_segment.is_empty() {
            wrapped_lines.push(current_segment);
            current_segment = character.to_string();
        } else {
            current_segment = candidate;
        }
    }

    if !current_segment.is_empty() {
        wrapped_lines.push(current_segment);
    }
}
