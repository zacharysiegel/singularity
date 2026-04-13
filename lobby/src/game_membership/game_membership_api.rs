use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::game_membership::GameMembershipSerial;
use sqlx::PgPool;

use crate::game::game_db;
use crate::game::game_model::Game;
use crate::http;
use crate::session::session_extractor::AuthenticatedAccount;
use super::game_membership_db;
use super::game_membership_model::GameMembership;

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/game/{game_id}/member").route(web::post().to(join_game)),
    );
}

async fn join_game(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
    auth: AuthenticatedAccount,
) -> HttpResponse {
    let game_id = unwrap_or_400!(uuid::Uuid::parse_str(&path.into_inner()));

    let game_entity = unwrap_or_500!(game_db::get_game_by_id(pool.get_ref(), game_id).await);
    let game_entity = unwrap_or_404!(game_entity);
    let game = unwrap_or_500!(Game::try_from(game_entity));

    // Only allow joining pending or active games
    if game.status == shared::schema::game::GameStatus::Completed {
        return HttpResponse::BadRequest().finish();
    }

    // Check max players not exceeded
    let member_count = unwrap_or_500!(game_membership_db::count_memberships_by_game(pool.get_ref(), game_id).await);
    if member_count >= i64::from(game.max_players) {
        return HttpResponse::Conflict().finish();
    }

    // Check not already a member
    let existing = unwrap_or_500!(
        game_membership_db::get_membership(pool.get_ref(), game_id, auth.account_id).await
    );
    if existing.is_some() {
        return HttpResponse::Conflict().finish();
    }

    let entity = unwrap_or_500!(
        game_membership_db::create_membership(pool.get_ref(), game_id, auth.account_id).await
    );

    let membership = GameMembership::from(entity);
    let serial = GameMembershipSerial::from(&membership);
    http::serialize_response(&request, &serial)
}
