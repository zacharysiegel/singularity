use crate::stage::StageState;
use std::sync::RwLock;

pub static STATE: State = State {
    frame_counter: RwLock::new(0),
    stage: StageState::DEFAULT,
};

#[derive(Debug)]
pub struct State {
    pub frame_counter: RwLock<u64>,
    pub stage: StageState,
}
