use super::game_membership_db;
use super::game_membership_model::{GameMembership, GameMembershipEntity};
use crate::game::game_db;
use crate::game::game_model::{Game, GameEntity};
use crate::http;
use crate::lobby_error::{LobbyError, OptionExt, ResultExt};
use crate::session::session_extractor::AuthenticatedAccount;
use actix_web::{HttpRequest, HttpResponse, web};
use shared::schema::game::GameStatus;
use shared::schema::game_membership::GameMembershipSerial;
use sqlx::PgPool;
use uuid::Uuid;

pub fn game_configurer(config: &mut web::ServiceConfig) {
    config.service(web::resource("/{game_id}/member").route(web::post().to(join_game)));
}

async fn join_game(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (game_id_string,): (String,) = path.into_inner();
    let game_id: Uuid = Uuid::parse_str(&game_id_string).or_bad_request()?;

    let game_entity: Option<GameEntity> = game_db::get_game_by_id(pool.get_ref(), game_id).await?;
    let game_entity: GameEntity = game_entity.or_not_found()?;
    let game: Game = Game::try_from(game_entity)?;

    if game.status != GameStatus::Pending {
        return Err(LobbyError::conflict(&format!(
            "Only {} games may be joined",
            GameStatus::Pending
        )));
    }

    let entity_opt: Option<GameMembershipEntity> =
        game_membership_db::create_membership_if_available(pool.get_ref(), game_id, auth.account_id).await?;

    let entity: GameMembershipEntity = match entity_opt {
        Some(entity) => entity,
        None => {
            return Err(LobbyError::conflict(
                "Game cannot be joined. (The game may be full or the account may already be a member.)",
            ));
        }
    };

    let membership: GameMembership = GameMembership::from(entity);
    let serial: GameMembershipSerial = GameMembershipSerial::from(&membership);
    Ok(http::serialize_response(&request, &serial))
}
