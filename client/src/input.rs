use raylib::ffi;
use raylib::ffi::{IsKeyPressed, IsMouseButtonPressed, IsMouseButtonReleased};
use std::ffi::c_int;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
}

pub fn handle_user_input() {
    if unsafe { IsKeyPressed(ffi::KeyboardKey::KEY_A as i32) } {
        log::debug!("a pressed");
    }

    if unsafe { IsMouseButtonReleased(MouseButton::Left as c_int) }{
        log::debug!("mouse left released");
    }
    if unsafe { IsMouseButtonReleased(MouseButton::Right as c_int) }{
        log::debug!("mouse right released");
    }
    if unsafe { IsMouseButtonReleased(MouseButton::Middle as c_int) }{
        log::debug!("mouse middle released");
    }
    if unsafe { IsMouseButtonPressed(MouseButton::Left as c_int) }{
        log::debug!("mouse left pressed");
    }
    if unsafe { IsMouseButtonPressed(MouseButton::Right as c_int) }{
        log::debug!("mouse right pressed");
    }
    if unsafe { IsMouseButtonPressed(MouseButton::Middle as c_int) }{
        log::debug!("mouse middle pressed");
    }
}
