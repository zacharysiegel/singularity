use crate::shader::StandardShader;
use crate::{math, new_standard_shader};
use raylib::math::Rectangle;
use raylib::prelude::RaylibShader;
use raylib::{RaylibHandle, RaylibThread};

const EXIT_ICON: &str = include_str!("../../shader/exit_icon.fs.glsl");

pub struct ExitIconShader {
    pub standard: StandardShader,
    pub u_button_size: i32,
    pub u_button_origin: i32,
}

impl ExitIconShader {
    pub fn new(rl: &mut RaylibHandle, rl_thread: &RaylibThread) -> Self {
        let standard: StandardShader = new_standard_shader!(rl, rl_thread, None, Some(EXIT_ICON));
        let u_button_size: i32 = standard.shader.borrow_mut().get_shader_location("u_button_size");
        let u_button_origin: i32 = standard.shader.borrow_mut().get_shader_location("u_button_origin");
        ExitIconShader {
            standard,
            u_button_size,
            u_button_origin,
        }
    }

    pub fn set_values(&self, rl: &RaylibHandle, container: Rectangle) {
        self.standard.set_values(rl);
        self.standard.shader.borrow_mut().set_shader_value(self.u_button_origin, math::rect_origin(container));
        self.standard.shader.borrow_mut().set_shader_value(self.u_button_size, container.width)
    }
}
