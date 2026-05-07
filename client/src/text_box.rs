use crate::font::DEFAULT_FONT_SPACING;
use crate::input::{
    CharPressHandler, CharPressResult,
    ClickHandler, ClickResult, HoverHandler, HoverResult, KeyPressHandler, KeyPressResult,
};
use raylib::consts::KeyboardKey;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::text::RaylibFont;
use raylib::RaylibHandle;
use shared::color::{TEXT_COLOR, WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR};
use shared::defaults::DEFAULT_RECTANGLE;
use shared::map::RenderCoord;
use shared::math;

const TEXT_BOX_FONT_SIZE: f32 = 16.;
const DEFAULT_PADDING: f32 = 6.;
const CURSOR_BLINK_FRAMES: u64 = 30;

#[derive(Debug)]
pub struct TextBox {
    pub rectangle: Rectangle,
    pub text: String,
    pub focused: bool,
    pub hovered: bool,
    pub padding: f32,
    pub on_submit: Option<fn(&str)>,

    frame_counter: u64,
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
    fn key_press(&mut self, _rl: &mut RaylibHandle, key: KeyboardKey) -> KeyPressResult {
        if !self.focused {
            return KeyPressResult::Pass;
        }
        match key {
            KeyboardKey::KEY_BACKSPACE => {
                self.text.pop();
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
        let mut candidate: String = self.text.clone();
        candidate.push(ch);
        let available_width: f32 = self.rectangle.width - self.padding * 2.;
        let candidate_width: f32 = rl.get_font_default()
            .measure_text(&candidate, TEXT_BOX_FONT_SIZE, DEFAULT_FONT_SPACING).x;
        if candidate_width <= available_width {
            self.text = candidate;
        }
        CharPressResult::Consume
    }
}

shared::default_const_impl!(TextBox);

impl TextBox {
    pub const DEFAULT: TextBox = TextBox {
        rectangle: DEFAULT_RECTANGLE,
        text: String::new(),
        focused: false,
        hovered: false,
        padding: DEFAULT_PADDING,
        on_submit: None,
        frame_counter: 0,
    };

    pub fn new(rectangle: Rectangle, text: &str) -> Self {
        TextBox {
            rectangle,
            text: String::from(text),
            ..Self::DEFAULT
        }
    }

    pub fn new_empty(rectangle: Rectangle) -> Self {
        Self::new(rectangle, "")
    }

    pub fn tick(&mut self) {
        self.frame_counter = self.frame_counter.wrapping_add(1);
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

        let text_x: f32 = position.x + self.padding;
        let text_y: f32 = position.y + (dimensions.y - TEXT_BOX_FONT_SIZE) / 2.;
        rl_draw.draw_text_ex(
            rl_draw.get_font_default(),
            &self.text,
            Vector2 { x: text_x, y: text_y },
            TEXT_BOX_FONT_SIZE,
            DEFAULT_FONT_SPACING,
            TEXT_COLOR,
        );

        if self.focused && (self.frame_counter / CURSOR_BLINK_FRAMES) % 2 == 0 {
            let text_measure: Vector2 =
                rl_draw.get_font_default().measure_text(&self.text, TEXT_BOX_FONT_SIZE, DEFAULT_FONT_SPACING);
            let cursor_x: f32 = text_x + text_measure.x + DEFAULT_FONT_SPACING;
            let cursor_y_top: f32 = text_y;
            let cursor_y_bottom: f32 = text_y + TEXT_BOX_FONT_SIZE;
            rl_draw.draw_line(
                cursor_x as i32,
                cursor_y_top as i32,
                cursor_x as i32,
                cursor_y_bottom as i32,
                TEXT_COLOR,
            );
        }
    }
}
