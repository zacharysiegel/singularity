use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;

use crate::lobby_error::LobbyError;
use crate::session::session_extractor::AuthenticatedAccount;
use shared::schema::ws_message::ConnectionType;
use crate::ws::handle;

const CONNECTION_TYPE: ConnectionType = ConnectionType::Lobby;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(web::resource(format!("/ws/{CONNECTION_TYPE}")).route(web::get().to(lobby_ws_handler)));
}

async fn lobby_ws_handler(
    request: HttpRequest,
    body: web::Payload,
    auth: AuthenticatedAccount,
    pg_pool: web::Data<PgPool>,
) -> Result<HttpResponse, LobbyError> {
    handle::ws_handler(request, body, auth, pg_pool, CONNECTION_TYPE, None).await
}
