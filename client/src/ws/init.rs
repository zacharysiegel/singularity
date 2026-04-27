use shared::environment::RuntimeEnvironment;
use shared::error::AppError;
use crate::state::STATE;
use crate::ws;

pub fn init() {
    match RuntimeEnvironment::default().is_debug() {
        true => init_debug(),
        false => {} // TODO: authenticate via login UI, then connect lobby WS with the real session token
    }
}

pub fn init_debug() {
    tokio::spawn(async move {
        let runtime_env: RuntimeEnvironment = RuntimeEnvironment::default();
        let auth: Result<String, AppError> = ws::auth::debug_authenticate(&runtime_env.lobby_http_origin()).await;

        match auth {
            Ok(token) => {
                log::info!("Debug authentication successful");
                *STATE.ws.token.write().unwrap() = Some(token.clone());
                ws::connect_lobby(&runtime_env.lobby_ws_origin(), &token);
            }
            Err(error) => {
                log::error!("Debug authentication failed: {error}");
            }
        }
    });
}
