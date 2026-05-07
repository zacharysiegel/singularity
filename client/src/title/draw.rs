use crate::button::RectangularButton;
use crate::config::APPLICATION_NAME;
use crate::state::{Loading, STATE};
use crate::title::TITLE_VERTICAL_MARGIN;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use shared::color::{MAP_BACKGROUND_COLOR, TEXT_COLOR};
use shared::math;
use std::sync::RwLockReadGuard;

pub fn draw(rl_draw: &mut RaylibDrawHandle) {
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
    let debug_button_guard: RwLockReadGuard<Option<RectangularButton>> =
        STATE.stage.title.debug_button.button.read().unwrap();
    let Some(debug_button) = debug_button_guard.as_ref() else {
        return;
    };

    let debug_ready: RwLockReadGuard<Loading> = STATE.stage.title.debug_button.loading.read().unwrap();
    if *debug_ready == Loading::Incomplete {
        debug_button.draw_with_text_override(rl_draw, "Loading...");
    } else {
        debug_button.draw(rl_draw);
    }
}

fn draw_main_buttons(rl_draw: &mut RaylibDrawHandle) {
    for button_lock in &STATE.stage.title.main_buttons {
        let button: RwLockReadGuard<RectangularButton> = button_lock.read().unwrap();
        button.draw(rl_draw);
    }
}
