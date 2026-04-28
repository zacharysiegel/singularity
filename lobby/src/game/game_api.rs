use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::game::{CreateGameRequest, GameBrowserEntry, GameBrowserQuery, GameSerial, GameStatus, UpdateGameStatusRequest};
use sqlx::PgPool;
use uuid::Uuid;
use crate::conversation::conversation_db;
use crate::lobby_error::{LobbyError, OptionExtLobbyError, ResultExtLobbyError};
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
            .route("/{game_id}", web::get().to(get_game))
            .route("/{game_id}/status", web::patch().to(update_game_status))
            .configure(crate::game_membership::game_membership_api::game_configurer)
            .configure(crate::game_session::game_session_api::game_configurer)
            .configure(crate::accolade::accolade_api::game_configurer)
            .configure(crate::statistic::statistic_api::game_configurer)
            .configure(crate::conversation::live::game_configurer),
    );
}

async fn list_games(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<GameBrowserQuery>,
) -> Result<HttpResponse, LobbyError> {
    let game_browser_rows: Vec<GameBrowserRow> = game_db::list_games(pool.get_ref(), query.status).await?;

    let game_browser_entries: Vec<GameBrowserEntry> = game_browser_rows
        .into_iter()
        .map(GameBrowserEntry::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(http::serialize_response(&request, &game_browser_entries))
}

async fn create_game(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let payload: CreateGameRequest = http::deserialize_request(&request, &body).or_bad_request()?;

    if !payload.is_valid() {
        return Err(LobbyError::bad_request("invalid create game request"));
    }

    let id: Uuid = Uuid::now_v7();
    let max_players: i32 = payload.max_players.unwrap_or(DEFAULT_MAX_PLAYERS);

    let entity: GameEntity =
        game_db::create_game(pool.get_ref(), id, &payload.name, auth.account_id, max_players).await?;

    game_membership_db::create_membership_if_available(pool.get_ref(), id, auth.account_id).await?;

    let game: Game = Game::try_from(entity)?;
    let serial: GameSerial = GameSerial::from(&game);
    Ok(http::serialize_response(&request, &serial))
}

async fn get_game(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, LobbyError> {
    let (game_id_string,): (String,) = path.into_inner();
    let game_id: Uuid = Uuid::parse_str(&game_id_string).or_bad_request()?;

    let entity: Option<GameEntity> = game_db::get_game_by_id(pool.get_ref(), game_id).await?;
    let entity: GameEntity = entity.or_not_found()?;

    let game: Game = Game::try_from(entity)?;
    let serial: GameSerial = GameSerial::from(&game);
    Ok(http::serialize_response(&request, &serial))
}

async fn update_game_status(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (game_id_string,): (String,) = path.into_inner();
    let game_id: Uuid = Uuid::parse_str(&game_id_string).or_bad_request()?;

    let payload: UpdateGameStatusRequest = http::deserialize_request(&request, &body).or_bad_request()?;

    let game_entity: Option<GameEntity> = game_db::get_game_by_id(pool.get_ref(), game_id).await?;
    let game_entity: GameEntity = game_entity.or_not_found()?;

    if game_entity.creator_id != auth.account_id {
        return Err(LobbyError::forbidden("only the game creator can update game status"));
    }

    let current_status: GameStatus = GameStatus::try_from(game_entity.status)?;
    validate_status_transition(current_status, payload.status)?;

    let updated_entity: GameEntity = game_db::update_game_status(pool.get_ref(), game_id, payload.status as i32).await?;

    if payload.status == GameStatus::Active {
        auto_create_game_conversation(pool.get_ref(), game_id, &game_entity.name, auth.account_id).await?;
    }

    let game: Game = Game::try_from(updated_entity)?;
    let serial: GameSerial = GameSerial::from(&game);
    Ok(http::serialize_response(&request, &serial))
}

fn validate_status_transition(current: GameStatus, target: GameStatus) -> Result<(), LobbyError> {
    let valid: bool = matches!(
        (current, target),
        (GameStatus::Pending, GameStatus::Active) | (GameStatus::Active, GameStatus::Completed)
    );
    if !valid {
        return Err(LobbyError::bad_request(&format!(
            "invalid status transition; [{current}] -> [{target}]"
        )));
    }
    Ok(())
}

async fn auto_create_game_conversation(
    pool: &PgPool,
    game_id: Uuid,
    game_name: &str,
    creator_account_id: Uuid,
) -> Result<(), LobbyError> {
    let member_ids: Vec<Uuid> = game_membership_db::get_member_account_ids(pool, game_id).await?;
    let conversation_name: String = format!("Global [{}]", game_name);

    // creator_account_id has no ownership semantics — it is simply the first member added
    // to the conversation. All members are equal once added.
    conversation_db::create_conversation(
        pool,
        Some(&conversation_name),
        Some(game_id),
        creator_account_id,
        &member_ids,
    )
    .await?;

    Ok(())
}
