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
        "select * from account where id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn get_account_by_email(pool: &PgPool, email: &str) -> Result<Option<AccountEntity>, AppError> {
    let record = sqlx::query_as::<_, AccountEntity>(
        "select * from account \
         where email = $1 and deleted_at is null",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn update_account(
    pool: &PgPool,
    id: Uuid,
    username: Option<&str>,
    email: Option<&str>,
) -> Result<AccountEntity, AppError> {
    let record = sqlx::query_as::<_, AccountEntity>(
        "update account \
         set username = coalesce($2, username), \
             email = coalesce($3, email), \
             updated = now() \
         where id = $1 and deleted_at is null \
         returning *",
    )
    .bind(id)
    .bind(username)
    .bind(email)
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn update_password_hash(pool: &PgPool, id: Uuid, password_hash: &str) -> Result<(), AppError> {
    sqlx::query(
        "update account \
         set password_hash = $2, updated = now() \
         where id = $1 and deleted_at is null",
    )
    .bind(id)
    .bind(password_hash)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn soft_delete_account(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    // Anonymize PII and mark as deleted in a single statement
    sqlx::query(
        "update account \
         set email = 'deleted-' || id::text || '@anonymized', \
             username = 'deleted-' || id::text, \
             password_hash = '', \
             deleted_at = now(), \
             updated = now() \
         where id = $1 and deleted_at is null",
    )
    .bind(id)
    .execute(pool)
    .await?;

    // Hard delete all sessions for the account
    sqlx::query("delete from session where account_id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
