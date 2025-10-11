use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::state::{Hex, STATE};
use crate::window::hex::HexWindow;
use raylib::consts::MouseButton;
use raylib::ffi;
use raylib::ffi::{GetMousePosition, IsKeyPressed, IsMouseButtonReleased};
use raylib::math::Vector2;
use std::ffi::c_int;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub fn handle_user_input() {
    if unsafe { IsKeyPressed(ffi::KeyboardKey::KEY_A as c_int) } {
        log::debug!("a pressed");
    }

    if unsafe { IsMouseButtonReleased(MouseButton::MOUSE_BUTTON_LEFT as c_int) } {
        select_hex();
    }
}

fn select_hex() {
    let map_origin: RwLockReadGuard<MapCoord> = STATE.map_origin.read().unwrap();
    let mouse_position: RenderCoord = RenderCoord(Vector2::from(unsafe { GetMousePosition() }));
    let containing_hex: Hex = mouse_position.containing_hex(&*map_origin);

    let mut hex_window: RwLockWriteGuard<HexWindow> = STATE.windows.hex.write().unwrap();
    hex_window.open(RenderCoord(Vector2::from(mouse_position)), containing_hex);
    drop(hex_window);
}
