use crate::button::RectangularButton;
use crate::color::{TEXT_COLOR, WINDOW_BACKGROUND_COLOR};
use crate::config::APPLICATION_NAME;
use crate::font::DEFAULT_FONT_SPACING;
use crate::title::DEBUG_TEXT;
use crate::{math, title};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use shared::environment::RuntimeEnvironment;

pub fn draw_title(rl_draw: &mut RaylibDrawHandle) {
    draw_title_text(rl_draw);

    if RuntimeEnvironment::default().is_debug() {
        draw_debug_button(rl_draw);
    }
}

fn draw_title_text(rl_draw: &mut RaylibDrawHandle) {
    const FONT_SIZE: f32 = 40.;
    const FONT_SPACING: f32 = 4.;
    let text: String = {
        let mut text: String = String::from(APPLICATION_NAME);
        text.replace_range(0..1, &text[0..1].to_uppercase());
        text
    };
    let position: Vector2 = math::centered_text_origin(
        Vector2 {
            x: (rl_draw.get_screen_width() / 2) as f32,
            y: (rl_draw.get_screen_height() / 2) as f32,
        },
        &text,
        rl_draw.get_font_default(),
        FONT_SIZE,
        FONT_SPACING,
    );

    rl_draw.draw_text_ex(
        rl_draw.get_font_default(),
        &text,
        position,
        FONT_SIZE,
        FONT_SPACING,
        TEXT_COLOR,
    );
}

fn draw_debug_button(rl_draw: &mut RaylibDrawHandle) {
    const FONT_SIZE: f32 = 18.;

    let button: RectangularButton = title::debug_button(rl_draw);
    let position: Vector2 = math::rect_origin(button.rectangle);
    let dimensions: Vector2 = math::rect_dimensions(button.rectangle);

    rl_draw.draw_rectangle_v(position, dimensions, WINDOW_BACKGROUND_COLOR);
    rl_draw.draw_text_ex(
        rl_draw.get_font_default(),
        DEBUG_TEXT,
        math::centered_text_origin(
            Vector2 {
                x: position.x + dimensions.x / 2.,
                y: position.y + dimensions.y / 2.,
            },
            DEBUG_TEXT,
            rl_draw.get_font_default(),
            FONT_SIZE,
            DEFAULT_FONT_SPACING,
        ),
        FONT_SIZE,
        DEFAULT_FONT_SPACING,
        TEXT_COLOR,
    );
}
