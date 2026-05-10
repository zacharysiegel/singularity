use crate::component::icon::{draw_close_x, draw_hamburger, draw_plus};
use crate::conversation::panel::{ChatPanel, RailButton};
use crate::state::STATE;
use crate::window::{BORDER_GAP, BORDER_THICKNESS, BUTTON_INTERNAL_MARGIN, BUTTON_WIDTH};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibThread;
use shared::color::{
    DIFF_HOVER_BUTTON, TEXT_COLOR, WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR, WINDOW_INTERIOR_BORDER_COLOR,
};
use std::f32::consts::SQRT_2;
use raylib::color::Color;
use shared::math;
use strum::IntoEnumIterator;

const PANEL_BACKGROUND_ALPHA: u8 = 0xE0;

pub fn draw(rl_draw: &mut RaylibDrawHandle, _rl_thread: &RaylibThread) {
    if !STATE.conversation.chat_panel.read().unwrap().open {
        return;
    }

    let panel_rect: Rectangle = ChatPanel::panel_rectangle(rl_draw.get_screen_width() as f32, rl_draw.get_screen_height() as f32);
    draw_panel_background(rl_draw, panel_rect);
    draw_rail_action_buttons(rl_draw, panel_rect);
}

fn draw_panel_background(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle) {
    let background_color: Color = Color {
        a: PANEL_BACKGROUND_ALPHA,
        ..WINDOW_BACKGROUND_COLOR
    };
    rl_draw.draw_rectangle_rec(panel_rect, background_color);
    draw_interior_border(rl_draw, panel_rect);
    rl_draw.draw_rectangle_lines_ex(panel_rect, BORDER_THICKNESS, WINDOW_BORDER_COLOR);
}

fn draw_interior_border(rl_draw: &mut RaylibDrawHandle, rect: Rectangle) {
    rl_draw.draw_line_ex(
        Vector2 { x: rect.x, y: rect.y + BORDER_GAP },
        Vector2 { x: rect.x + rect.width, y: rect.y + BORDER_GAP },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 { x: rect.x, y: rect.y + rect.height - BORDER_GAP },
        Vector2 { x: rect.x + rect.width, y: rect.y + rect.height - BORDER_GAP },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 { x: rect.x + BORDER_GAP, y: rect.y },
        Vector2 { x: rect.x + BORDER_GAP, y: rect.y + rect.height },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 { x: rect.x + rect.width - BORDER_GAP, y: rect.y },
        Vector2 { x: rect.x + rect.width - BORDER_GAP, y: rect.y + rect.height },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}

fn draw_rail_action_buttons(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle) {
    let hovered_button: Option<RailButton> = STATE.conversation.chat_panel.read().unwrap().hovered_rail_button;

    let buttons: Vec<(Rectangle, RailButton)> = RailButton::iter()
        .map(|button| (ChatPanel::rail_button_rect(panel_rect, button), button))
        .collect();

    draw_button_backgrounds(rl_draw, &buttons, hovered_button);

    let close_rect: Rectangle = buttons[0].0;
    let new_rect: Rectangle = buttons[1].0;
    let list_rect: Rectangle = buttons[2].0;

    draw_close_x(
        rl_draw,
        Vector2 {
            x: close_rect.x + close_rect.width / 2.,
            y: close_rect.y + close_rect.height / 2.,
        },
        (BUTTON_WIDTH / 2. - BUTTON_INTERNAL_MARGIN.x) * SQRT_2,
        4.5,
    );
    draw_plus(rl_draw, new_rect, 2., TEXT_COLOR);
    draw_hamburger(rl_draw, list_rect, 2., TEXT_COLOR);

    let rectangles: Vec<Rectangle> = buttons.iter().map(|(rect, _)| *rect).collect();
    draw_button_borders(rl_draw, &rectangles);
    draw_double_separator(rl_draw, close_rect.x, list_rect.y + list_rect.height);
}

fn draw_button_backgrounds(
    rl_draw: &mut RaylibDrawHandle,
    buttons: &[(Rectangle, RailButton)],
    hovered: Option<RailButton>,
) {
    for (rect, button) in buttons {
        let mut background_color: Color = WINDOW_BACKGROUND_COLOR;
        if hovered == Some(*button) {
            background_color = math::color_add(&background_color, &DIFF_HOVER_BUTTON);
        }
        rl_draw.draw_rectangle_rec(*rect, background_color);
    }
}

fn draw_button_borders(rl_draw: &mut RaylibDrawHandle, rectangles: &[Rectangle]) {
    for rect in rectangles {
        let vertices: [Vector2; 4] = [
            Vector2 { x: rect.x, y: rect.y },
            Vector2 { x: rect.x, y: rect.y + rect.height },
            Vector2 { x: rect.x + rect.width, y: rect.y + rect.height },
            Vector2 { x: rect.x + rect.width, y: rect.y },
        ];
        for i in 0..vertices.len() {
            rl_draw.draw_line_ex(
                vertices[i],
                vertices[(i + 1) % vertices.len()],
                BORDER_THICKNESS,
                WINDOW_INTERIOR_BORDER_COLOR,
            );
        }
        draw_button_accent(rl_draw, *rect);
    }
}

fn draw_button_accent(rl_draw: &mut RaylibDrawHandle, rect: Rectangle) {
    const ACCENT_HEIGHT: f32 = 10.;
    rl_draw.draw_line_ex(
        Vector2 { x: rect.x + rect.width, y: rect.y + rect.height - ACCENT_HEIGHT },
        Vector2 { x: rect.x + rect.width - ACCENT_HEIGHT, y: rect.y + rect.height },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}

fn draw_double_separator(rl_draw: &mut RaylibDrawHandle, x: f32, y: f32) {
    rl_draw.draw_line_ex(
        Vector2 { x, y },
        Vector2 { x: x + BUTTON_WIDTH, y },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 { x, y: y + 3. },
        Vector2 { x: x + BUTTON_WIDTH, y: y + 3. },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}
