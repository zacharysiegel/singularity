use chrono::{DateTime, Utc};
use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::session_model::SessionEntity;

pub async fn create_session(
    pool: &PgPool,
    id: Uuid,
    account_id: Uuid,
    token: &str,
    expires: DateTime<Utc>,
) -> Result<SessionEntity, AppError> {
    let record = sqlx::query_as::<_, SessionEntity>(
        "insert into session (id, account_id, token, expires) \
         values ($1, $2, $3, $4) \
         returning *",
    )
    .bind(id)
    .bind(account_id)
    .bind(token)
    .bind(expires)
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn get_session_by_token(pool: &PgPool, token: &str) -> Result<Option<SessionEntity>, AppError> {
    let record = sqlx::query_as::<_, SessionEntity>(
        "select s.id, s.account_id, s.token, s.created, s.expires \
         from session s \
         where s.token = $1 \
           and s.expires > now()",
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn refresh_session(
    pool: &PgPool,
    token: &str,
    new_expires: DateTime<Utc>,
) -> Result<(), AppError> {
    sqlx::query("update session set expires = $1 where token = $2")
        .bind(new_expires)
        .bind(token)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_session_by_token(pool: &PgPool, token: &str) -> Result<(), AppError> {
    sqlx::query("delete from session where token = $1")
        .bind(token)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_sessions_by_account(pool: &PgPool, account_id: Uuid) -> Result<(), AppError> {
    sqlx::query("delete from session where account_id = $1")
        .bind(account_id)
        .execute(pool)
        .await?;
    Ok(())
}
