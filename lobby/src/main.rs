use actix_web;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use environment::RuntimeEnvironment;
use lobby::db;
use log::LevelFilter;
use shared::environment;
use shared::error::AppError;
use sqlx::{PgPool, Pool, Postgres};
use web::Data;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    environment::load_env()?;
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .format_source_path(true)
        .try_init()
        .map_err(|e| AppError::from_error(&e.to_string(), Box::new(e)))?;

    log::info!("Runtime environment: {:?}", RuntimeEnvironment::default());

    let pgpool: Pool<Postgres> = db::sqlx_connect().await?;
    open_server(pgpool).await?;
    Ok(())
}

async fn open_server(pgpool: PgPool) -> Result<(), AppError> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .app_data(Data::new(pgpool.clone()))
            .configure(lobby::health::configurer)
            .configure(lobby::account::account_api::configurer)
            .configure(lobby::session::session_api::configurer)
            .configure(lobby::game::game_api::configurer)
            .configure(lobby::game_membership::game_membership_api::configurer)
            .configure(lobby::game_session::game_session_api::configurer)
            .configure(lobby::game_result::game_result_api::configurer)
            .default_service(web::route().to(HttpResponse::NotFound))
    })
    .bind("127.0.0.1:10000")?
    .run()
    .await
    .map_err(|e| AppError::from(e))
}
