use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::account_model::AccountEntity;

pub async fn create_account(
    pool: &PgPool,
    id: Uuid,
    email: &str,
    username: &str,
    password_hash: &str,
) -> Result<AccountEntity, AppError> {
    let record = sqlx::query_as::<_, AccountEntity>(
        "insert into account (id, email, username, password_hash) \
         values ($1, $2, $3, $4) \
         returning *",
    )
    .bind(id)
    .bind(email)
    .bind(username)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn get_account_by_id(pool: &PgPool, id: Uuid) -> Result<Option<AccountEntity>, AppError> {
    let record = sqlx::query_as::<_, AccountEntity>(
        "select a.id, a.email, a.username, a.password_hash, a.created, a.updated \
         from account a \
         where a.id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn get_account_by_email(pool: &PgPool, email: &str) -> Result<Option<AccountEntity>, AppError> {
    let record = sqlx::query_as::<_, AccountEntity>(
        "select a.id, a.email, a.username, a.password_hash, a.created, a.updated \
         from account a \
         where a.email = $1",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(record)
}
