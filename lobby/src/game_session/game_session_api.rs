use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::game_session::GameSessionSerial;
use sqlx::PgPool;
use uuid::Uuid;
use crate::game_membership::game_membership_db;
use crate::game_membership::game_membership_model::GameMembershipEntity;
use crate::http;
use crate::session::session_extractor::AuthenticatedAccount;
use super::game_session_db;
use super::game_session_model::{GameSession, GameSessionEntity};

pub fn configurer(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/game/{game_id}/enter").route(web::post().to(enter_game)))
        .service(web::resource("/game/{game_id}/exit").route(web::post().to(exit_game)));
}

async fn enter_game(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    let (game_id_string,): (String,) = path.into_inner();
    let game_id: Uuid = unwrap_or_400!(Uuid::parse_str(&game_id_string));

    let membership: Option<GameMembershipEntity> = unwrap_or_500!(
        game_membership_db::get_membership(pool.get_ref(), game_id, auth.account_id).await
    );
    if membership.is_none() {
        return HttpResponse::Forbidden().body("User must be a member of the game to enter it");
    }

    let existing: Option<GameSessionEntity> = unwrap_or_500!(
        game_session_db::get_active_game_session(pool.get_ref(), game_id, auth.account_id).await
    );
    if existing.is_some() {
        return HttpResponse::Conflict().body("User has already entered the game");
    }

    let entity: GameSessionEntity = unwrap_or_500!(
        game_session_db::create_game_session(pool.get_ref(), game_id, auth.account_id, auth.session_id).await
    );

    let game_session: GameSession = GameSession::from(entity);
    let serial: GameSessionSerial = GameSessionSerial::from(&game_session);
    http::serialize_response(&request, &serial)
}

async fn exit_game(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    let (game_id_string,): (String,) = path.into_inner();
    let game_id: Uuid = unwrap_or_400!(Uuid::parse_str(&game_id_string));

    let entity: Option<GameSessionEntity> = unwrap_or_500!(
        game_session_db::exit_game_session(pool.get_ref(), game_id, auth.account_id).await
    );
    let entity: GameSessionEntity = unwrap_or_404!(entity);

    let game_session: GameSession = GameSession::from(entity);
    let serial: GameSessionSerial = GameSessionSerial::from(&game_session);
    http::serialize_response(&request, &serial)
}
