use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::game::{CreateGameRequest, GameSerial};
use sqlx::PgPool;

use crate::http;
use crate::session::session_extractor::AuthenticatedAccount;
use super::game_db;
use super::game_model::Game;

const DEFAULT_MAX_PLAYERS: i32 = 8;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/game")
            .route("", web::post().to(create_game))
            .route("/{game_id}", web::get().to(get_game)),
    );
}

async fn create_game(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    let payload: CreateGameRequest = unwrap_or_400!(http::deserialize_request(&request, &body));

    if payload.name.is_empty() {
        return HttpResponse::BadRequest().finish();
    }

    let id = uuid::Uuid::now_v7();
    let max_players = payload.max_players.unwrap_or(DEFAULT_MAX_PLAYERS);

    let entity = unwrap_or_500!(
        game_db::create_game(pool.get_ref(), id, &payload.name, auth.account_id, max_players).await
    );

    let game = unwrap_or_500!(Game::try_from(entity));
    let serial = GameSerial::from(&game);
    http::serialize_response(&request, &serial)
}

async fn get_game(request: HttpRequest, pool: web::Data<PgPool>, path: web::Path<String>) -> HttpResponse {
    let game_id = unwrap_or_400!(uuid::Uuid::parse_str(&path.into_inner()));

    let entity = unwrap_or_500!(game_db::get_game_by_id(pool.get_ref(), game_id).await);
    let entity = unwrap_or_404!(entity);

    let game = unwrap_or_500!(Game::try_from(entity));
    let serial = GameSerial::from(&game);
    http::serialize_response(&request, &serial)
}
