use crate::component::text::{DEFAULT_FONT_SPACING, Text};
use crate::input::{
    CharPressHandler, CharPressResult, ClickHandler, ClickResult, HoverHandler, HoverResult, KeyPressHandler,
    KeyPressResult,
};
use raylib::RaylibHandle;
use raylib::color::Color;
use raylib::consts::KeyboardKey;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibScissorModeExt};
use raylib::math::{Rectangle, Vector2};
use raylib::text::RaylibFont;
use shared::color::{WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR, WINDOW_BORDER_FOCUSED_COLOR};
use shared::defaults::DEFAULT_RECTANGLE;
use shared::map::RenderCoord;
use shared::primitive;
use std::time::Instant;

const DEFAULT_TEXT_BOX_FONT_SIZE: f32 = 16.;
const DEFAULT_HORIZONTAL_PADDING: f32 = 6.;
const CURSOR_BLINK_CYCLE_MS: u128 = 1000;
const CURSOR_VISIBLE_RATIO: f64 = 0.6;

#[derive(Debug)]
pub struct TextBox {
    pub rectangle: Rectangle,
    pub text: Text,
    pub focused: bool,
    pub hovered: bool,
    pub horizontal_padding: f32,
    pub on_submit: Option<fn(&str)>,

    /// Cursor position is expressed as character units from the start of the string
    cursor_position: usize,
    scroll_offset: f32,
    created_at: Instant,
}

impl ClickHandler for TextBox {
    fn click(
        &mut self,
        _rl: &mut RaylibHandle,
        press_position: RenderCoord,
        release_position: RenderCoord,
    ) -> ClickResult {
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

impl HoverHandler for TextBox {
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

impl KeyPressHandler for TextBox {
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
                }
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_DELETE => {
                let char_count: usize = self.text.content.chars().count();
                if self.cursor_position < char_count {
                    primitive::remove_char(&mut self.text.content, self.cursor_position);
                }
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_LEFT => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    self.clamp_scroll_to_cursor(rl);
                }
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_RIGHT => {
                let char_count: usize = self.text.content.chars().count();
                if self.cursor_position < char_count {
                    self.cursor_position += 1;
                    self.clamp_scroll_to_cursor(rl);
                }
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_HOME => {
                self.cursor_position = 0;
                self.clamp_scroll_to_cursor(rl);
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_END => {
                self.cursor_position = self.text.content.chars().count();
                self.clamp_scroll_to_cursor(rl);
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_ENTER => {
                if let Some(on_submit) = self.on_submit {
                    on_submit(&self.text.content);
                }
                KeyPressResult::Consume
            }
            _ => KeyPressResult::Pass,
        }
    }
}

impl CharPressHandler for TextBox {
    fn char_press(&mut self, rl: &mut RaylibHandle, ch: char) -> CharPressResult {
        if !self.focused {
            return CharPressResult::Pass;
        }

        primitive::insert_char(&mut self.text.content, self.cursor_position, ch);
        self.cursor_position += 1;
        self.clamp_scroll_to_cursor(rl);
        CharPressResult::Consume
    }
}

impl Default for TextBox {
    fn default() -> Self {
        TextBox {
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
            on_submit: None,
            cursor_position: 0,
            scroll_offset: 0.,
            created_at: Instant::now(),
        }
    }
}

impl TextBox {
    pub fn new(rectangle: Rectangle, content: &str) -> Self {
        let mut text_box: TextBox = TextBox::default();
        text_box.rectangle = rectangle;
        text_box.text.content = String::from(content);
        text_box.cursor_position = content.chars().count();
        text_box
    }

    pub fn new_empty(rectangle: Rectangle) -> Self {
        Self::new(rectangle, "")
    }

    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle) {
        rl_draw.draw_rectangle_rec(self.rectangle, WINDOW_BACKGROUND_COLOR);

        let (border_color, border_thickness): (Color, f32) = if self.focused {
            (WINDOW_BORDER_FOCUSED_COLOR, 2.)
        } else {
            (WINDOW_BORDER_COLOR, 1.)
        };
        rl_draw.draw_rectangle_lines_ex(self.rectangle, border_thickness, border_color);

        let content_x: f32 = self.rectangle.x + self.horizontal_padding;
        let text_y: f32 = self.rectangle.y + (self.rectangle.height - self.text.font_size) / 2.;

        rl_draw.draw_scissor_mode(
            content_x as i32,
            self.rectangle.y as i32,
            self.inner_width() as i32,
            self.rectangle.height as i32,
            |mut scissor_draw| {
                let text_x: f32 = content_x - self.scroll_offset;

                scissor_draw.draw_text_ex(
                    scissor_draw.get_font_default(),
                    &self.text.content,
                    Vector2 { x: text_x, y: text_y },
                    self.text.font_size,
                    self.text.font_spacing,
                    self.text.color,
                );

                if self.cursor_visible() && self.focused {
                    let text_before_cursor: String = self.text.content.chars().take(self.cursor_position).collect();
                    let cursor_offset: f32 = scissor_draw
                        .get_font_default()
                        .measure_text(&text_before_cursor, self.text.font_size, self.text.font_spacing)
                        .x;
                    let cursor_x: f32 = text_x + cursor_offset + self.text.font_spacing;
                    scissor_draw.draw_line(
                        cursor_x as i32,
                        text_y as i32,
                        cursor_x as i32,
                        (text_y + self.text.font_size) as i32,
                        self.text.color,
                    );
                }
            },
        );
    }

    fn cursor_visible(&self) -> bool {
        let elapsed_ms: u128 = self.created_at.elapsed().as_millis();
        let position_in_cycle: u128 = elapsed_ms % CURSOR_BLINK_CYCLE_MS;
        let visible_duration_ms: u128 = (CURSOR_BLINK_CYCLE_MS as f64 * CURSOR_VISIBLE_RATIO) as u128;
        position_in_cycle < visible_duration_ms
    }

    fn inner_width(&self) -> f32 {
        self.rectangle.width - self.horizontal_padding * 2.
    }

    fn cursor_x_offset(&self, rl: &RaylibHandle) -> f32 {
        let text_before_cursor: String = self.text.content.chars().take(self.cursor_position).collect();
        rl.get_font_default().measure_text(&text_before_cursor, self.text.font_size, self.text.font_spacing).x
            + self.text.font_spacing
    }

    fn clamp_scroll_to_cursor(&mut self, rl: &RaylibHandle) {
        let cursor_x: f32 = self.cursor_x_offset(rl);
        let inner_width: f32 = self.inner_width();

        if cursor_x - self.scroll_offset > inner_width {
            self.scroll_offset = cursor_x - inner_width;
        }
        if cursor_x - self.scroll_offset < 0. {
            self.scroll_offset = cursor_x;
        }
        self.scroll_offset = self.scroll_offset.max(0.);
    }
}
