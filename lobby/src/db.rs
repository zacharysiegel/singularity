use shared::decrypt;
use shared::environment::RuntimeEnvironment;
use shared::error::AppError;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn sqlx_connect() -> Result<Pool<Postgres>, AppError> {
    let database_url: String = dotenvy::var("DATABASE_URL")?;
    let pool: Pool<Postgres> = PgPoolOptions::new().max_connections(16).connect(&database_url).await?;
    Ok(pool)
}

#[allow(unused)]
fn get_db_password() -> Result<String, AppError> {
    let runtime_environment: RuntimeEnvironment = RuntimeEnvironment::default();
    let password_key: String = format!(
        "postgres__user.singularity.password.{}",
        runtime_environment.to_string()
    );

    let password: Vec<u8> = decrypt::master_decrypt(&password_key)?;
    let password: String = String::from_utf8(password)?;
    Ok(password)
}
