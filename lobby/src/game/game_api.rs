use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::game::{CreateGameRequest, GameBrowserEntry, GameBrowserQuery, GameSerial};
use sqlx::PgPool;

use crate::game_membership::game_membership_db;
use crate::http;
use crate::session::session_extractor::AuthenticatedAccount;
use super::game_db;
use super::game_model::{Game, GameBrowserRow, GameEntity};

const DEFAULT_MAX_PLAYERS: i32 = 8;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/game")
            .route("", web::get().to(list_games))
            .route("", web::post().to(create_game))
            .route("/{game_id}", web::get().to(get_game)),
    );
}

async fn list_games(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<GameBrowserQuery>,
) -> HttpResponse {
    let game_browser_rows: Vec<GameBrowserRow> = unwrap_or_500!(game_db::list_games(pool.get_ref(), query.status).await);

    let game_browser_entries: Vec<GameBrowserEntry> = unwrap_or_500!(game_browser_rows
        .into_iter()
        .map(GameBrowserEntry::try_from)
        .collect::<Result<Vec<_>, _>>());

    http::serialize_response(&request, &game_browser_entries)
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

    let id: uuid::Uuid = uuid::Uuid::now_v7();
    let max_players: i32 = payload.max_players.unwrap_or(DEFAULT_MAX_PLAYERS);

    let entity: GameEntity = unwrap_or_500!(
        game_db::create_game(pool.get_ref(), id, &payload.name, auth.account_id, max_players).await
    );

    unwrap_or_500!(
        game_membership_db::create_membership_if_available(pool.get_ref(), id, auth.account_id).await
    );

    let game: Game = unwrap_or_500!(Game::try_from(entity));
    let serial: GameSerial = GameSerial::from(&game);
    http::serialize_response(&request, &serial)
}

async fn get_game(request: HttpRequest, pool: web::Data<PgPool>, path: web::Path<(String,)>) -> HttpResponse {
    let (game_id_string,): (String,) = path.into_inner();
    let game_id: uuid::Uuid = unwrap_or_400!(uuid::Uuid::parse_str(&game_id_string));

    let entity: Option<GameEntity> = unwrap_or_500!(game_db::get_game_by_id(pool.get_ref(), game_id).await);
    let entity: GameEntity = unwrap_or_404!(entity);

    let game: Game = unwrap_or_500!(Game::try_from(entity));
    let serial: GameSerial = GameSerial::from(&game);
    http::serialize_response(&request, &serial)
}
