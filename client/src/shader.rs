use raylib::{RaylibHandle, RaylibThread};

const BLUR: &str = include_str!("../shader/blur.glsl");

pub fn init(rl: &mut RaylibHandle, rl_thread: &RaylibThread) {
    log::debug!("{}", BLUR);
    rl.load_shader(rl_thread, None, None);
}
