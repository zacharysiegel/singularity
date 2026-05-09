use crate::component::icon::{draw_close_x, draw_hamburger, draw_plus};
use crate::conversation::panel::ChatPanel;
use crate::state::STATE;
use crate::window::{BORDER_THICKNESS, BUTTON_INTERNAL_MARGIN, BUTTON_WIDTH};
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibThread;
use shared::color::{
    TEXT_COLOR, WINDOW_BACKGROUND_COLOR, WINDOW_BORDER_COLOR, WINDOW_INTERIOR_BORDER_COLOR,
};
use std::f32::consts::SQRT_2;
use std::sync::RwLockReadGuard;

const PANEL_BACKGROUND_ALPHA: u8 = 0xD0;

pub fn draw(rl_draw: &mut RaylibDrawHandle, _rl_thread: &RaylibThread) {
    let chat_panel: RwLockReadGuard<ChatPanel> = STATE.conversation.chat_panel.read().unwrap();
    if !chat_panel.open {
        return;
    }

    let screen_width: f32 = rl_draw.get_screen_width() as f32;
    let screen_height: f32 = rl_draw.get_screen_height() as f32;
    let panel_rect: Rectangle = ChatPanel::panel_rectangle(screen_width, screen_height);

    draw_panel_background(rl_draw, panel_rect);
    draw_rail_action_buttons(rl_draw, panel_rect);
}

fn draw_panel_background(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle) {
    let background_color: raylib::color::Color = raylib::color::Color {
        a: PANEL_BACKGROUND_ALPHA,
        ..WINDOW_BACKGROUND_COLOR
    };
    rl_draw.draw_rectangle_rec(panel_rect, background_color);
    rl_draw.draw_rectangle_lines_ex(panel_rect, BORDER_THICKNESS, WINDOW_BORDER_COLOR);
}

fn draw_rail_action_buttons(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle) {
    let rail_x: f32 = panel_rect.x + panel_rect.width - BUTTON_WIDTH;

    let close_rect: Rectangle = Rectangle {
        x: rail_x,
        y: panel_rect.y,
        width: BUTTON_WIDTH,
        height: BUTTON_WIDTH,
    };
    let new_rect: Rectangle = Rectangle {
        x: rail_x,
        y: panel_rect.y + BUTTON_WIDTH,
        width: BUTTON_WIDTH,
        height: BUTTON_WIDTH,
    };
    let list_rect: Rectangle = Rectangle {
        x: rail_x,
        y: panel_rect.y + BUTTON_WIDTH * 2.,
        width: BUTTON_WIDTH,
        height: BUTTON_WIDTH,
    };

    draw_button_backgrounds(rl_draw, &[close_rect, new_rect, list_rect]);
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
    draw_button_borders(rl_draw, &[close_rect, new_rect, list_rect]);
    draw_double_separator(rl_draw, rail_x, list_rect.y + list_rect.height);
}

fn draw_button_backgrounds(rl_draw: &mut RaylibDrawHandle, rectangles: &[Rectangle]) {
    for rect in rectangles {
        rl_draw.draw_rectangle_rec(*rect, WINDOW_BACKGROUND_COLOR);
    }
}

fn draw_button_borders(rl_draw: &mut RaylibDrawHandle, rectangles: &[Rectangle]) {
    for rect in rectangles {
        rl_draw.draw_rectangle_lines_ex(*rect, BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);
    }
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
