use crate::button::RectangularButton;
use crate::color::{DIFF_HOVER_BUTTON, MAP_BACKGROUND_COLOR, TEXT_COLOR, WINDOW_BACKGROUND_COLOR};
use crate::config::APPLICATION_NAME;
use crate::font::DEFAULT_FONT_SPACING;
use crate::math;
use crate::state::STATE;
use crate::title::{BUTTON_FONT_SIZE, TITLE_VERTICAL_MARGIN};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use std::sync::RwLockReadGuard;

pub fn draw_title(rl_draw: &mut RaylibDrawHandle) {
    rl_draw.clear_background(MAP_BACKGROUND_COLOR);

    draw_title_text(rl_draw);
    draw_debug_button(rl_draw);
    draw_main_buttons(rl_draw);
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
            y: (rl_draw.get_screen_height() / 2) as f32 - TITLE_VERTICAL_MARGIN / 2.,
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
    let debug_button_l: RwLockReadGuard<Option<RectangularButton>> = STATE.stage.title.debug_button.read().unwrap();
    if (&debug_button_l).is_none() {
        return;
    }

    let debug_button: &RectangularButton = (*debug_button_l).as_ref().unwrap();
    draw_button(rl_draw, debug_button);
}

fn draw_main_buttons(rl_draw: &mut RaylibDrawHandle) {
    for button_l in &STATE.stage.title.main_buttons {
        let button: RwLockReadGuard<RectangularButton> = button_l.read().unwrap();
        draw_button(rl_draw, &button);
    }
}

fn draw_button(rl_draw: &mut RaylibDrawHandle, button: &RectangularButton) {
    let position: Vector2 = math::rect_origin(button.rectangle);
    let dimensions: Vector2 = math::rect_dimensions(button.rectangle);
    let mut bg_color: Color = WINDOW_BACKGROUND_COLOR;
    if button.is_hovered() {
        bg_color = math::color_add(&bg_color, &DIFF_HOVER_BUTTON);
    }

    rl_draw.draw_rectangle_v(position, dimensions, bg_color);
    if let Some(text) = &button.text {
        rl_draw.draw_text_ex(
            rl_draw.get_font_default(),
            text,
            math::centered_text_origin(
                Vector2 {
                    x: position.x + dimensions.x / 2.,
                    y: position.y + dimensions.y / 2.,
                },
                text,
                rl_draw.get_font_default(),
                BUTTON_FONT_SIZE,
                DEFAULT_FONT_SPACING,
            ),
            BUTTON_FONT_SIZE,
            DEFAULT_FONT_SPACING,
            TEXT_COLOR,
        );
    }
}
