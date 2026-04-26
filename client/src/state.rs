use crate::conversation::state::ConversationState;
use crate::stage::StageState;
use crate::texture::ScreenRenderTexture;
use crate::ws::state::WsState;
use std::mem;
use std::sync::{LazyLock, RwLock};

pub static STATE: LazyLock<State> = LazyLock::new(|| State {
    stage: StageState::DEFAULT,
    frame_counter: RwLock::new(0),
    screen_texture: RwLock::new(unsafe { mem::zeroed() }),
    ws: WsState::new(),
    conversation: ConversationState::new(),
});

#[derive(Debug)]
pub struct State {
    pub stage: StageState,
    pub frame_counter: RwLock<u64>,
    pub screen_texture: RwLock<ScreenRenderTexture>,
    pub ws: WsState,
    pub conversation: ConversationState,
}

#[derive(Debug, PartialEq)]
pub enum Loading {
    Incomplete,
    Complete,
}
