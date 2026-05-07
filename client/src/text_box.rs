use crate::font::DEFAULT_FONT_SPACING;
use crate::input::{
    CharPressHandler, CharPressResult,
    ClickHandler, ClickResult, HoverHandler, HoverResult, KeyPressHandler, KeyPressResult,
};
use raylib::consts::KeyboardKey;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle, RaylibScissorModeExt};
use raylib::math::{Rectangle, Vector2};
use raylib::text::RaylibFont;
use raylib::RaylibHandle;
use shared::color::{TEXT_COLOR, WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR};
use shared::defaults::DEFAULT_RECTANGLE;
use shared::map::RenderCoord;
use shared::math;
use shared::primitive;
use std::time::Instant;

const TEXT_BOX_FONT_SIZE: f32 = 16.;
const DEFAULT_HORIZONTAL_PADDING: f32 = 6.;
const CURSOR_BLINK_INTERVAL_MS: u128 = 500;

#[derive(Debug)]
pub struct TextBox {
    pub rectangle: Rectangle,
    pub text: String,
    pub focused: bool,
    pub hovered: bool,
    pub horizontal_padding: f32,
    pub on_submit: Option<fn(&str)>,

    cursor_position: usize,
    scroll_offset: f32,
    created_at: Instant,
}

impl ClickHandler for TextBox {
    fn click(&mut self, _rl: &mut RaylibHandle, press_position: RenderCoord, release_position: RenderCoord) -> ClickResult {
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
                    let byte_index: usize = primitive::byte_offset_at(&self.text, self.cursor_position - 1);
                    self.text.remove(byte_index);
                    self.cursor_position -= 1;
                    self.clamp_scroll_to_cursor(rl);
                }
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_DELETE => {
                let char_count: usize = self.text.chars().count();
                if self.cursor_position < char_count {
                    let byte_index: usize = primitive::byte_offset_at(&self.text, self.cursor_position);
                    self.text.remove(byte_index);
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
                let char_count: usize = self.text.chars().count();
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
                self.cursor_position = self.text.chars().count();
                self.clamp_scroll_to_cursor(rl);
                KeyPressResult::Consume
            }
            KeyboardKey::KEY_ENTER => {
                if let Some(on_submit) = self.on_submit {
                    on_submit(&self.text);
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

        let byte_index: usize = primitive::byte_offset_at(&self.text, self.cursor_position);
        self.text.insert(byte_index, ch);
        self.cursor_position += 1;
        self.clamp_scroll_to_cursor(rl);
        CharPressResult::Consume
    }
}

impl Default for TextBox {
    fn default() -> Self {
        TextBox {
            rectangle: DEFAULT_RECTANGLE,
            text: String::new(),
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

    pub fn new(rectangle: Rectangle, text: &str) -> Self {
        let mut text_box: TextBox = TextBox::default();
        text_box.rectangle = rectangle;
        text_box.text = String::from(text);
        text_box.cursor_position = text.chars().count();
        text_box
    }

    pub fn new_empty(rectangle: Rectangle) -> Self {
        Self::new(rectangle, "")
    }

    pub fn draw(&self, rl_draw: &mut RaylibDrawHandle) {
        let position: Vector2 = math::rect_origin(self.rectangle);
        let dimensions: Vector2 = math::rect_dimensions(self.rectangle);

        rl_draw.draw_rectangle_v(position, dimensions, WINDOW_BACKGROUND_COLOR);

        let border_color: raylib::color::Color = if self.focused {
            TEXT_COLOR
        } else {
            WINDOW_BORDER_COLOR
        };
        rl_draw.draw_rectangle_lines(
            position.x as i32,
            position.y as i32,
            dimensions.x as i32,
            dimensions.y as i32,
            border_color,
        );

        let content_x: f32 = position.x + self.horizontal_padding;
        let content_width: i32 = (dimensions.x - self.horizontal_padding * 2.) as i32;
        let text_y: f32 = position.y + (dimensions.y - TEXT_BOX_FONT_SIZE) / 2.;

        rl_draw.draw_scissor_mode(
            content_x as i32,
            position.y as i32,
            content_width,
            dimensions.y as i32,
            |mut scissor_draw| {
                let text_x: f32 = content_x - self.scroll_offset;
                scissor_draw.draw_text_ex(
                    scissor_draw.get_font_default(),
                    &self.text,
                    Vector2 { x: text_x, y: text_y },
                    TEXT_BOX_FONT_SIZE,
                    DEFAULT_FONT_SPACING,
                    TEXT_COLOR,
                );

                if self.focused && self.cursor_visible() {
                    let text_before_cursor: String = self.text.chars().take(self.cursor_position).collect();
                    let cursor_offset: f32 = scissor_draw.get_font_default()
                        .measure_text(&text_before_cursor, TEXT_BOX_FONT_SIZE, DEFAULT_FONT_SPACING).x;
                    let cursor_x: f32 = text_x + cursor_offset + DEFAULT_FONT_SPACING;
                    scissor_draw.draw_line(
                        cursor_x as i32,
                        text_y as i32,
                        cursor_x as i32,
                        (text_y + TEXT_BOX_FONT_SIZE) as i32,
                        TEXT_COLOR,
                    );
                }
            },
        );
    }

    fn cursor_visible(&self) -> bool {
        let elapsed_ms: u128 = self.created_at.elapsed().as_millis();
        (elapsed_ms / CURSOR_BLINK_INTERVAL_MS) % 2 == 0
    }

    fn inner_width(&self) -> f32 {
        self.rectangle.width - self.horizontal_padding * 2.
    }

    fn cursor_x_offset(&self, rl: &RaylibHandle) -> f32 {
        let text_before_cursor: String = self.text.chars().take(self.cursor_position).collect();
        rl.get_font_default()
            .measure_text(&text_before_cursor, TEXT_BOX_FONT_SIZE, DEFAULT_FONT_SPACING).x
            + DEFAULT_FONT_SPACING
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
