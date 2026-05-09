use crate::conversation::panel::ChatPanel;
use crate::state::STATE;
use crate::window::{BORDER_THICKNESS, BUTTON_INTERNAL_MARGIN, BUTTON_WIDTH, draw_close_x};
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

    // Backgrounds
    for rect in [close_rect, new_rect, list_rect] {
        rl_draw.draw_rectangle_rec(rect, WINDOW_BACKGROUND_COLOR);
    }

    // Close X icon
    draw_close_x(
        rl_draw,
        Vector2 {
            x: close_rect.x + close_rect.width / 2.,
            y: close_rect.y + close_rect.height / 2.,
        },
        (BUTTON_WIDTH / 2. - BUTTON_INTERNAL_MARGIN.x) * SQRT_2,
        4.5,
    );

    // Plus icon
    let plus_center: Vector2 = Vector2 {
        x: new_rect.x + new_rect.width / 2.,
        y: new_rect.y + new_rect.height / 2.,
    };
    let plus_size: f32 = 10.;
    rl_draw.draw_line_ex(
        Vector2 { x: plus_center.x - plus_size, y: plus_center.y },
        Vector2 { x: plus_center.x + plus_size, y: plus_center.y },
        2.,
        TEXT_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 { x: plus_center.x, y: plus_center.y - plus_size },
        Vector2 { x: plus_center.x, y: plus_center.y + plus_size },
        2.,
        TEXT_COLOR,
    );

    // Hamburger icon (≡)
    let hamburger_x: f32 = list_rect.x + 12.;
    let hamburger_width: f32 = list_rect.width - 24.;
    for i in 0..3 {
        let line_y: f32 = list_rect.y + 14. + (i as f32 * 7.);
        rl_draw.draw_line_ex(
            Vector2 { x: hamburger_x, y: line_y },
            Vector2 { x: hamburger_x + hamburger_width, y: line_y },
            2.,
            TEXT_COLOR,
        );
    }

    // Borders (no margin between buttons)
    for rect in [close_rect, new_rect, list_rect] {
        rl_draw.draw_rectangle_lines_ex(rect, BORDER_THICKNESS, WINDOW_INTERIOR_BORDER_COLOR);
    }

    // Double border separator below action buttons
    let separator_y: f32 = list_rect.y + list_rect.height;
    rl_draw.draw_line_ex(
        Vector2 { x: rail_x, y: separator_y },
        Vector2 { x: rail_x + BUTTON_WIDTH, y: separator_y },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
    rl_draw.draw_line_ex(
        Vector2 { x: rail_x, y: separator_y + 3. },
        Vector2 { x: rail_x + BUTTON_WIDTH, y: separator_y + 3. },
        BORDER_THICKNESS,
        WINDOW_INTERIOR_BORDER_COLOR,
    );
}
