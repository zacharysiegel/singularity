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
const EXIT_ICON: &str = include_str!("../shader/exit_icon.fs.glsl");

macro_rules! new_standard_shader {
    ($rl:ident, $rl_thread:ident, $vertex_shader:expr, $fragment_shader:expr) => {{
        ::log::debug!(
            "Loading shaders; [{}, {}]",
            ::std::stringify!($vertex_shader),
            ::std::stringify!($fragment_shader),
        );

        let shader = ::raylib::RaylibHandle::load_shader_from_memory($rl, $rl_thread, $vertex_shader, $fragment_shader);
        let standard_shader = crate::shader::StandardShader::new(shader);

        if standard_shader.shader.borrow().locs.is_null() {
            ::std::panic!(
                "Failed to load shader; [{}, {}]",
                ::std::stringify!($vertex_shader),
                ::std::stringify!($fragment_shader),
            );
        }
        standard_shader
    }};
}

pub struct ShaderStore {
    pub blur: Rc<StandardShader>,
    pub fxaa: Rc<StandardShader>,
    pub exit_icon: Rc<StandardShader>,
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
    pub u_resolution: i32,
    pub u_mouse: i32,
    pub u_time: i32,
}

impl StandardUniforms {
    pub fn new(shader: &Shader) -> Self {
        StandardUniforms {
            u_resolution: shader.get_shader_location("u_resolution"),
            u_mouse: shader.get_shader_location("u_mouse"),
            u_time: shader.get_shader_location("u_time"),
        }
    }
}

pub fn init(rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
    let blur: StandardShader = new_standard_shader!(rl, rl_thread, None, Some(BLUR));
    let fxaa: StandardShader = new_standard_shader!(rl, rl_thread, None, Some(FXAA));
    let exit_icon: StandardShader = new_standard_shader!(rl, rl_thread, None, Some(EXIT_ICON));

    SHADER_STORE.replace(MaybeUninit::new({
        ShaderStore {
            blur: Rc::new(blur),
            fxaa: Rc::new(fxaa),
            exit_icon: Rc::new(exit_icon),
        }
    }));
}
