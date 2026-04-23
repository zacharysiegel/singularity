use chrono::{DateTime, Utc};
use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::session_model::SessionEntity;

pub async fn create_session(
    pool: &PgPool,
    account_id: Uuid,
    token: &str,
    expiry: DateTime<Utc>,
) -> Result<SessionEntity, AppError> {
    let record: SessionEntity = sqlx::query_as!(
        SessionEntity,
        "insert into session (account_id, token, expiry)
         values ($1, $2, $3)
         returning *",
        account_id,
        token,
        expiry,
    )
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn get_session_by_token(pool: &PgPool, token: &str) -> Result<Option<SessionEntity>, AppError> {
    let record: Option<SessionEntity> = sqlx::query_as!(
        SessionEntity,
        "select s.id, s.account_id, s.token, s.created, s.expiry
         from session s
         where s.token = $1
           and s.expiry > now()",
        token,
    )
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn refresh_session(
    pool: &PgPool,
    token: &str,
    new_expiry: DateTime<Utc>,
) -> Result<(), AppError> {
    sqlx::query!(
        "update session set expiry = $1 where token = $2",
        new_expiry,
        token,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_session_by_token(pool: &PgPool, token: &str) -> Result<(), AppError> {
    sqlx::query!("delete from session where token = $1", token)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_sessions_by_account(pool: &PgPool, account_id: Uuid) -> Result<(), AppError> {
    sqlx::query!("delete from session where account_id = $1", account_id)
        .execute(pool)
        .await?;
    Ok(())
}
