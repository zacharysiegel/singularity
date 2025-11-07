use actix_web;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use environment::RuntimeEnvironment;
use log::LevelFilter;
use shared::environment;
use shared::error::AppError;
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    environment::load_env()?;
    env_logger::builder().filter_level(LevelFilter::Debug).format_source_path(true).try_init()?;

    log::info!("Runtime environment: {:?}", RuntimeEnvironment::default());

    // todo: initialize database connection
    // let pgpool: Pool<Postgres> = inventory::db::sqlx_connect().await
    //     .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    open_server().await?;
    Ok(())
}

pub async fn open_server(/*pgpool: Pool<Postgres>*/) -> Result<(), AppError> {
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            // .app_data(web::Data::new(pgpool.clone()))
            .default_service(web::route().to(HttpResponse::NotFound))
        // example routing extension: .configure(crate::public_api::configurer)
    })
    .bind("127.0.0.1:10000")?
    .run()
    .await
    .map_err(|e| AppError::from(e))
}
