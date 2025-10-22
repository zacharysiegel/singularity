use raylib::prelude::RaylibShader;
use raylib::shaders::Shader;
use raylib::{RaylibHandle, RaylibThread};
use std::cell::RefCell;
use std::mem::MaybeUninit;
use std::rc::Rc;

thread_local! {
pub static SHADER_STORE: RefCell<MaybeUninit<ShaderStore>> = RefCell::new(MaybeUninit::uninit());
}

const BLUR: &str = include_str!("../shader/blur.fs.glsl");
const FXAA: &str = include_str!("../shader/fxaa.fs.glsl");

pub struct ShaderStore {
    pub blur: Rc<StandardShader>,
    pub fxaa: Rc<StandardShader>,
}

pub struct StandardShader {
    pub shader: RefCell<Shader>,
    pub uniforms: StandardUniforms,
}

impl StandardShader {
    pub fn new(shader: Shader) -> Self {
        StandardShader {
            uniforms: StandardUniforms::new(&shader),
            shader: RefCell::new(shader),
        }
    }
}

pub struct StandardUniforms {
    pub u_dimensions: i32,
    pub u_mouse: i32,
    pub u_time: i32,
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
    SHADER_STORE.replace(MaybeUninit::new({
        let blur: StandardShader = StandardShader::new(rl.load_shader_from_memory(rl_thread, None, Some(BLUR)));
        if blur.shader.borrow().locs.is_null() {
            panic!("Failed to load shader; [{}]", stringify!(blur));
        }

        let fxaa: StandardShader = StandardShader::new(rl.load_shader_from_memory(rl_thread, None, Some(FXAA)));
        if fxaa.shader.borrow().locs.is_null() {
            panic!("Failed to load shader; [{}]", stringify!(fxaa));
        }

        ShaderStore {
            blur: Rc::new(blur),
            fxaa: Rc::new(fxaa),
        }
    }));
}
