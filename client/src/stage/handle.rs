use super::StageType;
use crate::state::STATE;
use crate::ws;
use shared::environment::RuntimeEnvironment;
use shared::schema::ws_message::ConnectionType;
use std::sync::RwLockReadGuard;

pub fn handle_stage_transition(previous_stage: Option<StageType>, current_stage: StageType) {
    let Some(previous_stage) = previous_stage else {
        return;
    };
    log::info!("Stage transition: {previous_stage:?} -> {current_stage:?}");

    match (previous_stage, current_stage) {
        (_, StageType::Game) => {
            // TODO: catch up on in-game chat history via REST before WS delivers new events
            let runtime_environment: RuntimeEnvironment = RuntimeEnvironment::default();
            let lobby_ws_origin: String = runtime_environment.lobby_ws_origin();
            let token_guard: RwLockReadGuard<Option<String>> = STATE.ws.token.read().unwrap();
            if let Some(token) = &*token_guard {
                ws::connect(&lobby_ws_origin, token, ConnectionType::Live);
            }
        }
        (StageType::Game, _) => {
            ws::disconnect(ConnectionType::Live);
        }
        _ => {}
    }
}
