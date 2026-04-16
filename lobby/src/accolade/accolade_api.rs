use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::accolade::AccoladeSerial;
use sqlx::PgPool;

use crate::http;
use super::accolade_db;
use super::accolade_model::Accolade;

pub fn configurer(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/account/{account_id}/accolades")
                .route(web::get().to(get_account_accolades)),
        )
        .service(
            web::resource("/game/{game_id}/accolades")
                .route(web::get().to(get_game_accolades)),
        );
}

async fn get_account_accolades(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let account_id = unwrap_or_400!(uuid::Uuid::parse_str(&path.into_inner()));

    let entities = unwrap_or_500!(accolade_db::get_accolades_by_account(pool.get_ref(), account_id).await);
    let serials: Vec<AccoladeSerial> = unwrap_or_500!(entities
        .into_iter()
        .map(|entity| Accolade::try_from(entity).map(|accolade| AccoladeSerial::from(&accolade)))
        .collect::<Result<Vec<_>, _>>());

    http::serialize_response(&request, &serials)
}

async fn get_game_accolades(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<String>,
) -> HttpResponse {
    let game_id = unwrap_or_400!(uuid::Uuid::parse_str(&path.into_inner()));

    let entities = unwrap_or_500!(accolade_db::get_accolades_by_game(pool.get_ref(), game_id).await);
    let serials: Vec<AccoladeSerial> = unwrap_or_500!(entities
        .into_iter()
        .map(|entity| Accolade::try_from(entity).map(|accolade| AccoladeSerial::from(&accolade)))
        .collect::<Result<Vec<_>, _>>());

    http::serialize_response(&request, &serials)
}
