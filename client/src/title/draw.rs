use crate::component::button::RectangularButton;
use crate::component::vertical_scroll_region::VerticalScrollRegion;
use crate::component::text_wrap;
use crate::config::APPLICATION_NAME;
use crate::state::STATE;
use crate::component::text_input::TextInput;
use crate::title::{DEBUG_SCROLL_TEXT, TITLE_VERTICAL_MARGIN};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use shared::color::{MAP_BACKGROUND_COLOR, TEXT_COLOR, WINDOW_BORDER_COLOR};
use shared::math;
use std::sync::RwLockReadGuard;

pub fn draw(rl_draw: &mut RaylibDrawHandle) {
    rl_draw.clear_background(MAP_BACKGROUND_COLOR);

    draw_title_text(rl_draw);
    draw_debug_button(rl_draw);
    draw_debug_scroll_region(rl_draw);
    draw_debug_text_box(rl_draw);
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

    debug_button.draw(rl_draw);
}

fn draw_debug_scroll_region(rl_draw: &mut RaylibDrawHandle) {
    let scroll_guard: RwLockReadGuard<Option<VerticalScrollRegion>> =
        STATE.stage.title.debug_scroll_region.read().unwrap();
    let Some(scroll_region) = scroll_guard.as_ref() else {
        return;
    };

    rl_draw.draw_rectangle_lines_ex(scroll_region.viewport(), 1., WINDOW_BORDER_COLOR);

    let sample_text: &str = DEBUG_SCROLL_TEXT;

    let font_size: f32 = 14.;
    let font_spacing: f32 = 1.;
    let line_height: f32 = 18.;
    let wrap_width: f32 = scroll_region.viewport().width - scroll_region.padding() * 2.;
    let font = rl_draw.get_font_default();
    let wrapped_lines: Vec<String> = text_wrap::wrap_text(sample_text, &font, font_size, font_spacing, wrap_width);

    scroll_region.draw(rl_draw, |mut scissor_draw, y_offset| {
        for (line_index, line) in wrapped_lines.iter().enumerate() {
            let line_y: f32 = scroll_region.viewport().y + y_offset + (line_index as f32 * line_height);
            scissor_draw.draw_text_ex(
                scissor_draw.get_font_default(),
                line,
                Vector2 { x: scroll_region.viewport().x + scroll_region.padding(), y: line_y },
                font_size,
                font_spacing,
                TEXT_COLOR,
            );
        }
    });
}

fn draw_debug_text_box(rl_draw: &mut RaylibDrawHandle) {
    let text_box_guard: RwLockReadGuard<Option<TextInput>> = STATE.stage.title.debug_text_box.read().unwrap();
    let Some(text_box) = text_box_guard.as_ref() else {
        return;
    };
    text_box.draw(rl_draw);
}

fn draw_main_buttons(rl_draw: &mut RaylibDrawHandle) {
    for button_lock in &STATE.stage.title.main_buttons {
        let button: RwLockReadGuard<RectangularButton> = button_lock.read().unwrap();
        button.draw(rl_draw);
    }
}
