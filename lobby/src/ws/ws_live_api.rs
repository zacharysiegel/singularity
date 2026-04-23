use crate::lobby_error::LobbyError;
use crate::session::session_extractor::AuthenticatedAccount;
use crate::ws::connection_type::ConnectionType;
use crate::ws::handle;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use std::time::Duration;

const CONNECTION_TYPE: ConnectionType = ConnectionType::Live;
const RATE_LIMIT_MESSAGES_PER_SECOND: u64 = 10;
const RATE_LIMIT_INTERVAL: Duration = Duration::from_millis(1000 / RATE_LIMIT_MESSAGES_PER_SECOND);

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(web::resource(format!("/ws/{CONNECTION_TYPE}")).route(web::get().to(live_ws_handler)));
}

async fn live_ws_handler(
    request: HttpRequest,
    body: web::Payload,
    auth: AuthenticatedAccount,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, LobbyError> {
    handle::ws_handler(request, body, auth, pg_pool, ConnectionType::Live, Some(RATE_LIMIT_INTERVAL)).await
}
