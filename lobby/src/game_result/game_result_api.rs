use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::game_result::GameResultSerial;
use sqlx::PgPool;

use crate::http;
use super::game_result_db;
use super::game_result_model::GameResult;

pub fn configurer(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("/game/{game_id}/results").route(web::get().to(get_game_results)))
        .service(web::resource("/account/{account_id}/history").route(web::get().to(get_account_history)));
}

async fn get_game_results(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let game_id = unwrap_or_400!(uuid::Uuid::parse_str(&path.into_inner()));

    let entities = unwrap_or_500!(game_result_db::get_results_by_game(pool.get_ref(), game_id).await);
    let serials: Vec<GameResultSerial> = entities
        .into_iter()
        .map(|entity| GameResultSerial::from(&GameResult::from(entity)))
        .collect();

    http::serialize_response(&request, &serials)
}

async fn get_account_history(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let account_id = unwrap_or_400!(uuid::Uuid::parse_str(&path.into_inner()));

    let entities = unwrap_or_500!(game_result_db::get_results_by_account(pool.get_ref(), account_id).await);
    let serials: Vec<GameResultSerial> = entities
        .into_iter()
        .map(|entity| GameResultSerial::from(&GameResult::from(entity)))
        .collect();

    http::serialize_response(&request, &serials)
}
