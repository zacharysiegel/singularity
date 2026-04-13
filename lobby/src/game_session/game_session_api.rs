use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::game_session::GameSessionSerial;
use sqlx::PgPool;

use crate::game_membership::game_membership_db;
use crate::http;
use crate::session::session_extractor::AuthenticatedAccount;
use super::game_session_db;
use super::game_session_model::GameSession;

pub fn configurer(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/game/{game_id}/enter").route(web::post().to(enter_game)))
        .service(web::resource("/game/{game_id}/exit").route(web::post().to(exit_game)));
}

async fn enter_game(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    let game_id = unwrap_or_400!(uuid::Uuid::parse_str(&path.into_inner()));

    // Verify user is a member of this game
    let membership = unwrap_or_500!(
        game_membership_db::get_membership(pool.get_ref(), game_id, auth.account_id).await
    );
    if membership.is_none() {
        return HttpResponse::Forbidden().finish();
    }

    // Check for existing active game session (duplicate prevention)
    let existing = unwrap_or_500!(
        game_session_db::get_active_game_session(pool.get_ref(), game_id, auth.account_id).await
    );
    if existing.is_some() {
        return HttpResponse::Conflict().finish();
    }

    let id = uuid::Uuid::now_v7();
    let entity = unwrap_or_500!(
        game_session_db::create_game_session(pool.get_ref(), id, game_id, auth.account_id, auth.session_id).await
    );

    let game_session = GameSession::from(entity);
    let serial = GameSessionSerial::from(&game_session);
    http::serialize_response(&request, &serial)
}

async fn exit_game(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    let game_id = unwrap_or_400!(uuid::Uuid::parse_str(&path.into_inner()));

    let entity = unwrap_or_500!(
        game_session_db::exit_game_session(pool.get_ref(), game_id, auth.account_id).await
    );
    let entity = unwrap_or_404!(entity);

    let game_session = GameSession::from(entity);
    let serial = GameSessionSerial::from(&game_session);
    http::serialize_response(&request, &serial)
}
