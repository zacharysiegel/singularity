use crate::component::text::{DEFAULT_FONT_SPACING, Text};
use crate::input::{
    CharPressHandler, CharPressResult, ClickButton, ClickHandler, ClickResult, HoverHandler, HoverResult, KeyPressHandler,
    KeyPressResult,
};
use raylib::RaylibHandle;
use raylib::color::Color;
use raylib::consts::KeyboardKey;
use raylib::drawing::{RaylibDraw, RaylibScissorModeExt};
use raylib::math::{Rectangle, Vector2};
use raylib::text::{RaylibFont, WeakFont};
use shared::color::{DIFF_HOVER_BUTTON, WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR, WINDOW_BORDER_FOCUSED_COLOR};
use shared::defaults::DEFAULT_RECTANGLE;
use shared::map::RenderCoord;
use shared::primitive;
use std::time::Instant;

const DEFAULT_TEXT_BOX_FONT_SIZE: f32 = 16.;
const DEFAULT_HORIZONTAL_PADDING: f32 = 12.;
const CURSOR_BLINK_CYCLE_MS: u128 = 1500;
const CURSOR_VISIBLE_RATIO: f64 = 0.6;
const CURSOR_THICKNESS: f32 = 2.;

/// Extra pixels added to both edges of the scissor region to prevent clipping of glyph
/// overhang on characters like "j" which extend slightly beyond their origin point.
const GLYPH_OVERFLOW_MARGIN: f32 = 2.;

/// Visual border treatment for a `TextInput`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextInputBorderStyle {
    /// All four sides. Standard text-field affordance.
    All,
    /// Only the bottom edge. Suitable for embedded inputs where the surrounding container
    /// already provides side framing.
    Underlined,
    /// No border at all.
    None,
}

#[derive(Debug)]
pub struct TextInput {
    pub rectangle: Rectangle,
    pub text: Text,
    pub focused: bool,
    pub hovered: bool,
    pub horizontal_padding: f32,
    /// Extra reserved width on the trailing (right) edge beyond `horizontal_padding`. Use
    /// when an adjacent overlay control (e.g. a send button) sits on top of the input's
    /// right edge — text and cursor positioning will stop short of it.
    pub trailing_inset: f32,
    pub on_submit: Option<fn(&str)>,
    pub border_style: TextInputBorderStyle,
    /// Resting background color. Hover/focus states composite on top of this.
    pub background_color: Color,

    /// Cursor position is expressed as character units from the start of the string
    cursor_position: usize,
    scroll_offset: f32,
    last_input_at: Instant,
}

impl ClickHandler for TextInput {
    fn click(
        &mut self,
        _rl: &mut RaylibHandle,
        button: ClickButton,
        press_position: RenderCoord,
        release_position: RenderCoord,
    ) -> ClickResult {
        if button != ClickButton::Left {
            return ClickResult::Pass;
        }

        if self.rectangle.check_collision_point_rec(press_position)
            && self.rectangle.check_collision_point_rec(release_position)
        {
            self.focused = true;
            ClickResult::Consume
        } else {
            self.focused = false;
            ClickResult::Pass
        }
    }
}

impl HoverHandler for TextInput {
    fn hover(&mut self, _rl: &mut RaylibHandle, mouse_position: RenderCoord) -> HoverResult {
        if self.rectangle.check_collision_point_rec(mouse_position) {
            self.hovered = true;
            HoverResult::Consume
        } else {
            self.hovered = false;
            HoverResult::Pass
        }
    }
}

impl KeyPressHandler for TextInput {
    fn key_press(&mut self, rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        if !self.focused {
            return KeyPressResult::Pass;
        }

        match key {
            KeyboardKey::KEY_BACKSPACE => {
                if self.cursor_position > 0 {
                    primitive::remove_char(&mut self.text.content, self.cursor_position - 1);
                    self.cursor_position -= 1;
                    self.clamp_scroll_to_cursor(rl);
                    self.reset_cursor_blink();
                }
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_DELETE => {
                let char_count: usize = self.text.content.chars().count();
                if self.cursor_position < char_count {
                    primitive::remove_char(&mut self.text.content, self.cursor_position);
                    self.reset_cursor_blink();
                }
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_LEFT => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.clamp_scroll_to_cursor(rl);
                    self.reset_cursor_blink();
                }
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_RIGHT => {
                let char_count: usize = self.text.content.chars().count();
                if self.cursor_position < char_count {
                    self.cursor_position += 1;
                    self.clamp_scroll_to_cursor(rl);
                    self.reset_cursor_blink();
                }
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_HOME => {
                self.cursor_position = 0;
                self.clamp_scroll_to_cursor(rl);
                self.reset_cursor_blink();
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_END => {
                self.cursor_position = self.text.content.chars().count();
                self.clamp_scroll_to_cursor(rl);
                self.reset_cursor_blink();
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_ENTER => {
                if let Some(on_submit) = self.on_submit {
                    on_submit(&self.text.content);
                }
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_ESCAPE => {
                self.focused = false;
                KeyPressResult::Consume
            }
            _ => KeyPressResult::Pass,
        }
    }
}

impl CharPressHandler for TextInput {
    fn char_press(&mut self, rl: &mut RaylibHandle, ch: char) -> CharPressResult {
        if !self.focused {
            return CharPressResult::Pass;
        }

        primitive::insert_char(&mut self.text.content, self.cursor_position, ch);
        self.cursor_position += 1;
        self.clamp_scroll_to_cursor(rl);
        self.reset_cursor_blink();
        CharPressResult::Consume
    }
}

impl Default for TextInput {
    fn default() -> Self {
        TextInput {
            rectangle: DEFAULT_RECTANGLE,
            text: Text {
                content: String::new(),
                font_size: DEFAULT_TEXT_BOX_FONT_SIZE,
                font_spacing: DEFAULT_FONT_SPACING,
                color: shared::color::TEXT_COLOR,
            },
            focused: false,
            hovered: false,
            horizontal_padding: DEFAULT_HORIZONTAL_PADDING,
            trailing_inset: 0.,
            on_submit: None,
            border_style: TextInputBorderStyle::All,
            background_color: WINDOW_BACKGROUND_COLOR,
            cursor_position: 0,
            scroll_offset: 0.,
            last_input_at: Instant::now(),
        }
    }
}

impl TextInput {
    pub fn new(rectangle: Rectangle, content: &str) -> Self {
        let mut text_box: TextInput = TextInput::default();
        text_box.rectangle = rectangle;
        text_box.text.content = String::from(content);
        text_box.cursor_position = content.chars().count();
        text_box
    }

    pub fn new_empty(rectangle: Rectangle) -> Self {
        Self::new(rectangle, "")
    }

    pub fn clear(&mut self) {
        self.text.content.clear();
        self.cursor_position = 0;
        self.scroll_offset = 0.;
        self.reset_cursor_blink();
    }

    pub fn draw(&self, rl_draw: &mut impl RaylibDraw) {
        let mut background_color: Color = self.background_color;
        if !self.focused && self.hovered {
            background_color = shared::math::color_add(&background_color, &DIFF_HOVER_BUTTON);
        }
        rl_draw.draw_rectangle_rec(self.rectangle, background_color);

        let border_color: Color = if self.focused {
            WINDOW_BORDER_FOCUSED_COLOR
        } else {
            WINDOW_BORDER_COLOR
        };
        let border_thickness: f32 = 1.;
        match self.border_style {
            TextInputBorderStyle::All => {
                rl_draw.draw_rectangle_lines_ex(self.rectangle, border_thickness, border_color);
            }
            TextInputBorderStyle::Underlined => {
                let bottom_y: f32 = self.rectangle.y + self.rectangle.height;
                rl_draw.draw_line_ex(
                    Vector2 { x: self.rectangle.x, y: bottom_y },
                    Vector2 { x: self.rectangle.x + self.rectangle.width, y: bottom_y },
                    border_thickness,
                    border_color,
                );
            }
            TextInputBorderStyle::None => {}
        }

        let content_x: f32 = self.rectangle.x + self.horizontal_padding;
        let scissor_x: f32 = content_x - GLYPH_OVERFLOW_MARGIN;
        let scissor_width: i32 = (self.inner_width() + GLYPH_OVERFLOW_MARGIN * 2.) as i32;
        let text_y: f32 = self.rectangle.y + (self.rectangle.height - self.text.font_size) / 2.;
        let font: WeakFont = unsafe { WeakFont::from_raw(raylib::ffi::GetFontDefault()) };

        rl_draw.draw_scissor_mode(
            scissor_x as i32,
            self.rectangle.y as i32,
            scissor_width,
            self.rectangle.height as i32,
            |mut scissor_draw| {
                let text_x: f32 = content_x - self.scroll_offset;

                scissor_draw.draw_text_ex(
                    &font,
                    &self.text.content,
                    Vector2 { x: text_x, y: text_y },
                    self.text.font_size,
                    self.text.font_spacing,
                    self.text.color,
                );

                if self.cursor_visible() && self.focused {
                    let cursor_x: f32 = text_x + self.cursor_offset(&font);
                    let cursor_top: Vector2 = Vector2 { x: cursor_x, y: text_y };
                    let cursor_bottom: Vector2 = Vector2 { x: cursor_x, y: text_y + self.text.font_size };
                    scissor_draw.draw_line_ex(cursor_top, cursor_bottom, CURSOR_THICKNESS, self.text.color);
                }
            },
        );
    }

    fn reset_cursor_blink(&mut self) {
        self.last_input_at = Instant::now();
    }

    fn cursor_visible(&self) -> bool {
        let elapsed_ms: u128 = self.last_input_at.elapsed().as_millis();
        let position_in_cycle: u128 = elapsed_ms % CURSOR_BLINK_CYCLE_MS;
        let visible_duration_ms: u128 = (CURSOR_BLINK_CYCLE_MS as f64 * CURSOR_VISIBLE_RATIO) as u128;
        position_in_cycle < visible_duration_ms
    }

    fn cursor_offset(&self, font: &WeakFont) -> f32 {
        let text_before_cursor: String = self.text.content.chars().take(self.cursor_position).collect();
        let text_measure: Vector2 = font.measure_text(&text_before_cursor, self.text.font_size, self.text.font_spacing);
        if self.cursor_position == 0 {
            0.
        } else {
            text_measure.x + self.text.font_spacing / 2.
        }
    }

    fn inner_width(&self) -> f32 {
        self.rectangle.width - self.horizontal_padding * 2. - self.trailing_inset
    }

    fn clamp_scroll_to_cursor(&mut self, rl: &RaylibHandle) {
        let cursor_x: f32 = self.cursor_offset(&rl.get_font_default());
        let inner_width: f32 = self.inner_width();

        if cursor_x - self.scroll_offset > inner_width {
            self.scroll_offset = cursor_x - inner_width;
        } else if cursor_x - self.scroll_offset < 0. {
            self.scroll_offset = cursor_x;
        }

        self.scroll_offset = self.scroll_offset.max(0.);
    }
}
