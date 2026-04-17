use super::game_membership_db;
use super::game_membership_model::{GameMembership, GameMembershipEntity};
use crate::game::game_db;
use crate::game::game_model::{Game, GameEntity};
use crate::http;
use crate::session::session_extractor::AuthenticatedAccount;
use actix_web::{HttpRequest, HttpResponse, web};
use shared::schema::game::GameStatus;
use shared::schema::game_membership::GameMembershipSerial;
use sqlx::PgPool;
use uuid::Uuid;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(web::resource("/game/{game_id}/member").route(web::post().to(join_game)));
}

async fn join_game(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    let (game_id_string,): (String,) = path.into_inner();
    let game_id: Uuid = unwrap_or_400!(Uuid::parse_str(&game_id_string));

    let game_entity: Option<GameEntity> = unwrap_or_500!(game_db::get_game_by_id(pool.get_ref(), game_id).await);
    let game_entity: GameEntity = unwrap_or_404!(game_entity);
    let game: Game = unwrap_or_500!(Game::try_from(game_entity));

    if game.status != GameStatus::Pending {
        return HttpResponse::Conflict().body(format!("Only {} games may be joined", GameStatus::Pending));
    }

    let entity: Option<GameMembershipEntity> =
        unwrap_or_500!(game_membership_db::create_membership_if_available(pool.get_ref(), game_id, auth.account_id).await);

    let entity: GameMembershipEntity = match entity {
        Some(entity) => entity,
        None => return HttpResponse::Conflict().finish(),
    };

    let membership: GameMembership = GameMembership::from(entity);
    let serial: GameMembershipSerial = GameMembershipSerial::from(&membership);
    http::serialize_response(&request, &serial)
}
