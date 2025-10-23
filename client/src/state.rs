use crate::stage::StageState;
use crate::texture::ScreenRenderTexture;
use std::mem;
use std::sync::RwLock;

pub static STATE: State = State {
    stage: StageState::DEFAULT,
    frame_counter: RwLock::new(0),
    screen_texture: RwLock::new(unsafe { mem::zeroed() }),
};

#[derive(Debug)]
pub struct State {
    pub stage: StageState,
    pub frame_counter: RwLock<u64>,
    pub screen_texture: RwLock<ScreenRenderTexture>,
}
