use crate::map::coordinate::{MapCoord, RenderCoord};
use crate::state::STATE;
use raylib::ffi;
use raylib::ffi::{Color, DrawCircleV, GetMousePosition, IsKeyPressed, IsMouseButtonPressed, IsMouseButtonReleased};
use raylib::math::Vector2;
use std::ffi::c_int;
use std::sync::RwLockReadGuard;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
}

pub fn handle_user_input() {
    if unsafe { IsKeyPressed(ffi::KeyboardKey::KEY_A as c_int) } {
        log::debug!("a pressed");
    }

    if unsafe { IsMouseButtonReleased(MouseButton::Left as c_int) } {
        log::debug!("mouse left released");
    }
    if unsafe { IsMouseButtonReleased(MouseButton::Right as c_int) } {
        log::debug!("mouse right released");
    }
    if unsafe { IsMouseButtonReleased(MouseButton::Middle as c_int) } {
        log::debug!("mouse middle released");
    }
    if unsafe { IsMouseButtonPressed(MouseButton::Left as c_int) } {
        log::debug!("mouse left pressed");
    }
    if unsafe { IsMouseButtonPressed(MouseButton::Right as c_int) } {
        log::debug!("mouse right pressed");
    }
    if unsafe { IsMouseButtonPressed(MouseButton::Middle as c_int) } {
        log::debug!("mouse middle pressed");
    }

    let map_origin: RwLockReadGuard<MapCoord> = STATE.map_origin.read().unwrap();
    let mouse_position: RenderCoord = RenderCoord(Vector2::from(unsafe { GetMousePosition() }));
    let hex_position: RenderCoord =
        mouse_position.containing_hex(&*map_origin).hex_coord.map_coord().render_coord(&*map_origin);

    unsafe {
        DrawCircleV(
            hex_position.into(),
            9.,
            Color {
                r: 0x10,
                g: 0xf0,
                b: 0x80,
                a: 0xff,
            },
        );
    }
}
