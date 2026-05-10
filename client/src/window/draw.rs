use crate::component::button::RectangularButton;
use crate::component::frame::{draw_side_button_frame, draw_window_frame};
use crate::component::icon::draw_close_x;
use crate::state::STATE;
use crate::window::{ErrorWindow, HexWindow, PauseWindow, Window, BUTTON_WIDTH};
use raylib::drawing::RaylibDrawHandle;
use raylib::math::{Rectangle, Vector2};
use raylib::RaylibThread;
use shared::color::WINDOW_BACKGROUND_COLOR;
use shared::map::RenderCoord;
use std::f32::consts::SQRT_2;
use std::sync::RwLockReadGuard;

pub use crate::component::frame::{BORDER_GAP, BORDER_THICKNESS};
pub const BUTTON_INTERNAL_MARGIN: Vector2 = Vector2 { x: 11., y: 11. };

/// These windows are considered part of the "game" and will be blurred when an overlay window is active
pub fn draw_game_windows(rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
    let hex: RwLockReadGuard<HexWindow> = STATE.stage.game.window.hex.read().unwrap();
    hex.draw(rl_draw, rl_thread);
    drop(hex);
}

/// These windows are not considered part of the "game" and will not be blurred when an overlay window is active
pub fn draw_overlay_windows(rl_draw: &mut RaylibDrawHandle, rl_thread: &RaylibThread) {
    let pause: RwLockReadGuard<PauseWindow> = STATE.stage.game.window.pause.read().unwrap();
    pause.draw(rl_draw, rl_thread);
    drop(pause);

    let error: RwLockReadGuard<ErrorWindow> = STATE.stage.game.window.error.read().unwrap();
    // error.draw.rs(rl_draw, rl_thread);
    drop(error);
}

pub fn draw_window_base(rl_draw: &mut RaylibDrawHandle, window: &dyn Window) {
    draw_background(rl_draw, window);
    draw_close_button(rl_draw, window);
}

fn draw_background(rl_draw: &mut RaylibDrawHandle, window: &dyn Window) {
    let origin: RenderCoord = window.origin().unwrap();
    let full: Rectangle = Rectangle {
        x: origin.x,
        y: origin.y,
        width: window.dimensions().x,
        height: window.dimensions().y,
    };
    draw_window_frame(rl_draw, full, WINDOW_BACKGROUND_COLOR);
}

fn draw_close_button(rl_draw: &mut RaylibDrawHandle, window: &dyn Window) {
    let button: &RectangularButton = window.close_button();
    draw_side_button_frame(rl_draw, button.rectangle, button.is_hovered());
    draw_close_x(
        rl_draw,
        Vector2 {
            x: button.rectangle.x + button.rectangle.width / 2.,
            y: button.rectangle.y + button.rectangle.height / 2.,
        },
        (BUTTON_WIDTH / 2. - BUTTON_INTERNAL_MARGIN.x) * SQRT_2,
        4.5,
    )
}

pub fn draw_side_button_lines(rl_draw: &mut RaylibDrawHandle, button: &RectangularButton) {
    draw_side_button_frame(rl_draw, button.rectangle, false);
}
