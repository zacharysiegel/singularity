use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::statistic::StatisticSerial;
use sqlx::PgPool;

use crate::http;
use super::statistic_db;
use super::statistic_model::Statistic;

pub fn configurer(config: &mut web::ServiceConfig) {
    config
        .service(
            web::resource("/account/{account_id}/statistics")
                .route(web::get().to(get_account_statistics)),
        )
        .service(
            web::resource("/game/{game_id}/statistics")
                .route(web::get().to(get_game_statistics)),
        );
}

async fn get_account_statistics(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let (account_id_string,) = path.into_inner();
    let account_id = unwrap_or_400!(uuid::Uuid::parse_str(&account_id_string));

    let statistic_entities = unwrap_or_500!(statistic_db::get_statistics_by_account(pool.get_ref(), account_id).await);
    let statistic_serials: Vec<StatisticSerial> = unwrap_or_500!(statistic_entities
        .into_iter()
        .map(|entity| Statistic::try_from(entity).map(|statistic| StatisticSerial::from(&statistic)))
        .collect::<Result<Vec<_>, _>>());

    http::serialize_response(&request, &statistic_serials)
}

async fn get_game_statistics(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let (game_id_string,) = path.into_inner();
    let game_id = unwrap_or_400!(uuid::Uuid::parse_str(&game_id_string));

    let statistic_entities = unwrap_or_500!(statistic_db::get_statistics_by_game(pool.get_ref(), game_id).await);
    let statistic_serials: Vec<StatisticSerial> = unwrap_or_500!(statistic_entities
        .into_iter()
        .map(|entity| Statistic::try_from(entity).map(|statistic| StatisticSerial::from(&statistic)))
        .collect::<Result<Vec<_>, _>>());

    http::serialize_response(&request, &statistic_serials)
}
