pub const DEFAULT_FONT_SIZE: f32 = 18.;
pub const DEFAULT_FONT_SPACING: f32 = 2.;

#[derive(Debug)]
pub struct Text {
    pub content: String,
    pub font_size: f32,
    pub font_spacing: f32,
}

impl Text {
    pub fn new(content: &str, font_size: f32) -> Text {
        Text {
            content: content.to_string(),
            font_size,
            font_spacing: DEFAULT_FONT_SPACING,
        }
    }

    pub fn from_str_default(content: &str) -> Text {
        Text::new(content, DEFAULT_FONT_SIZE)
    }
}
