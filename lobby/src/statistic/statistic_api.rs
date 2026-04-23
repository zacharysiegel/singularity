use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::statistic::StatisticSerial;
use sqlx::PgPool;

use crate::error::{LobbyError, ResultExt};
use crate::http;
use super::statistic_db;
use super::statistic_model::{Statistic, StatisticEntity};

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
) -> Result<HttpResponse, LobbyError> {
    let (account_id_string,): (String,) = path.into_inner();
    let account_id: uuid::Uuid = uuid::Uuid::parse_str(&account_id_string).or_bad_request()?;

    let statistic_entities: Vec<StatisticEntity> =
        statistic_db::get_statistics_by_account(pool.get_ref(), account_id).await?;
    let statistic_serials: Vec<StatisticSerial> = statistic_entities
        .into_iter()
        .map(|entity| Statistic::try_from(entity).map(|statistic| StatisticSerial::from(&statistic)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(http::serialize_response(&request, &statistic_serials))
}

async fn get_game_statistics(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, LobbyError> {
    let (game_id_string,): (String,) = path.into_inner();
    let game_id: uuid::Uuid = uuid::Uuid::parse_str(&game_id_string).or_bad_request()?;

    let statistic_entities: Vec<StatisticEntity> =
        statistic_db::get_statistics_by_game(pool.get_ref(), game_id).await?;
    let statistic_serials: Vec<StatisticSerial> = statistic_entities
        .into_iter()
        .map(|entity| Statistic::try_from(entity).map(|statistic| StatisticSerial::from(&statistic)))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(http::serialize_response(&request, &statistic_serials))
}
