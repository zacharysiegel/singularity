use raylib::prelude::RaylibShader;
use raylib::shaders::Shader;
use raylib::{RaylibHandle, RaylibThread};
use std::sync::OnceLock;

pub const SHADER_STORE: OnceLock<ShaderStore> = OnceLock::new();

const BLUR: &str = include_str!("../shader/blur.fs.glsl");

pub struct ShaderStore {
    blur: StandardShader,
}

pub struct StandardShader {
    shader: Shader,
    uniforms: StandardUniforms,
}

impl StandardShader {
    pub fn new(shader: Shader) -> Self {
        StandardShader {
            uniforms: StandardUniforms::new(&shader),
            shader,
        }
    }
}

pub struct StandardUniforms {
    u_dimensions: i32,
    u_mouse: i32,
    u_time: i32,
}

impl StandardUniforms {
    pub fn new(shader: &Shader) -> Self {
        StandardUniforms {
            u_dimensions: shader.get_shader_location("u_dimensions"),
            u_mouse: shader.get_shader_location("u_mouse"),
            u_time: shader.get_shader_location("u_time"),
        }
    }
}

pub fn init(rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
    SHADER_STORE.get_or_init(|| ShaderStore {
        blur: StandardShader::new(rl.load_shader(rl_thread, None, Some(BLUR))),
    });
}
