use super::StageType;
use crate::state::STATE;
use crate::ws;
use shared::environment::RuntimeEnvironment;
use std::sync::RwLockReadGuard;

pub fn handle_stage_transition(previous_stage: Option<StageType>, current_stage: StageType) {
    let Some(previous_stage) = previous_stage else {
        return;
    };

    log::info!("Stage transition: {previous_stage:?} -> {current_stage:?}");

    let runtime_environment: RuntimeEnvironment = RuntimeEnvironment::default();
    let lobby_ws_url: &str = runtime_environment.get_lobby_ws_url();

    match (previous_stage, current_stage) {
        (_, StageType::Game) => {
            // TODO: catch up on in-game chat history via REST before WS delivers new events
            let token_guard: RwLockReadGuard<Option<String>> = STATE.ws.token.read().unwrap();
            if let Some(token) = &*token_guard {
                ws::connect_live(lobby_ws_url, token);
            }
        }
        (StageType::Game, _) => {
            ws::disconnect_live();
        }
        _ => {}
    }
}
