use crate::state::STATE;
use crate::{connect, ws};
use shared::environment::RuntimeEnvironment;
use shared::error::AppError;
use shared::schema::ws_message::ConnectionType;

pub fn init() {
    let runtime_env = RuntimeEnvironment::default();

    tokio::spawn(async move {
        let token: Result<String, AppError> = ws::auth::debug_authenticate(&runtime_env.lobby_http_origin()).await;
        let Ok(token) = token else {
            log::error!(
                "debug authentication failed, token not set [{}]",
                runtime_env.lobby_http_origin()
            );
            return;
        };
        *STATE.ws.token.write().unwrap() = Some(token.clone());

        let Ok(_) = connect::connect() else {
            log::error!("live server connection failed");
            return;
        };
        ws::connect(&runtime_env.lobby_ws_origin(), &token, ConnectionType::Lobby);
    });
}
