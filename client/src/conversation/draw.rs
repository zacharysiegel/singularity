use crate::component::frame::{BORDER_THICKNESS, draw_side_button_frame, draw_window_frame};
use crate::component::icon::{draw_close_x, draw_hamburger, draw_plus};
use crate::conversation::panel::{ChatPanel, RailButton};
use crate::state::STATE;
use crate::window::BUTTON_WIDTH;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibThread;
use shared::color::{TEXT_COLOR, WINDOW_BACKGROUND_COLOR, WINDOW_INTERIOR_BORDER_COLOR};
use strum::IntoEnumIterator;

const PANEL_BACKGROUND_ALPHA: u8 = 0xE8;

pub fn draw(rl_draw: &mut RaylibDrawHandle, _rl_thread: &RaylibThread) {
    if !STATE.conversation.chat_panel.read().unwrap().open {
        return;
    }

    let panel_rect: Rectangle = ChatPanel::panel_rectangle(rl_draw.get_screen_width() as f32, rl_draw.get_screen_height() as f32);
    draw_window_frame(rl_draw, panel_rect, Color {
        a: PANEL_BACKGROUND_ALPHA,
        ..WINDOW_BACKGROUND_COLOR
    });
    draw_rail_action_buttons(rl_draw, panel_rect);
}

fn draw_rail_action_buttons(rl_draw: &mut RaylibDrawHandle, panel_rect: Rectangle) {
    let hovered_button: Option<RailButton> = STATE.conversation.chat_panel.read().unwrap().hovered_rail_button;

    let buttons: Vec<(Rectangle, RailButton)> = RailButton::iter()
        .map(|button| (ChatPanel::rail_button_rect(panel_rect, button), button))
        .collect();

    for (rect, button) in &buttons {
        draw_side_button_frame(rl_draw, *rect, hovered_button == Some(*button));
    }

    let close_rect: Rectangle = buttons[0].0;
    let new_rect: Rectangle = buttons[1].0;
    let list_rect: Rectangle = buttons[2].0;

    draw_close_x(rl_draw, close_rect, 4.5, shared::color::RED);
    draw_plus(rl_draw, new_rect, 2., TEXT_COLOR);
    draw_hamburger(rl_draw, list_rect, 2., TEXT_COLOR);
    draw_double_separator(rl_draw, close_rect.x, list_rect.y + list_rect.height);
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
