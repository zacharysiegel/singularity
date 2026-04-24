use shared::error::AppError;
use sqlx::{PgPool, Postgres, Transaction};
use uuid::Uuid;

use super::account_model::AccountEntity;
use crate::follow::follow_db;
use crate::session::session_db;

pub async fn create_account(
    pool: &PgPool,
    email: &str,
    username: &str,
    password_hash: &str,
) -> Result<AccountEntity, AppError> {
    let record: AccountEntity = sqlx::query_as!(
        AccountEntity,
        "insert into account (email, username, password_hash)
         values ($1, $2, $3)
         returning *",
        email,
        username,
        password_hash,
    )
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn get_account_by_id(pool: &PgPool, id: Uuid) -> Result<Option<AccountEntity>, AppError> {
    let record: Option<AccountEntity> = sqlx::query_as!(
        AccountEntity,
        "select * from account where id = $1",
        id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn get_account_by_email(pool: &PgPool, email: &str) -> Result<Option<AccountEntity>, AppError> {
    let record: Option<AccountEntity> = sqlx::query_as!(
        AccountEntity,
        "select * from account
         where email = $1 and deleted_at is null",
        email,
    )
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
    let record: AccountEntity = sqlx::query_as!(
        AccountEntity,
        "update account
         set username = coalesce($2, username),
             email = coalesce($3, email),
             updated = now()
         where id = $1 and deleted_at is null
         returning *",
        id,
        username,
        email,
    )
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn update_password_hash(pool: &PgPool, id: Uuid, password_hash: &str) -> Result<(), AppError> {
    sqlx::query!(
        "update account
         set password_hash = $2, updated = now()
         where id = $1 and deleted_at is null",
        id,
        password_hash,
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn soft_delete_account(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let mut transaction: Transaction<Postgres> = pool.begin().await?;

    sqlx::query!(
        "update account
         set email = 'deleted-' || id::text || '@anonymized',
             username = 'deleted-' || id::text,
             password_hash = '',
             deleted_at = now(),
             updated = now()
         where id = $1 and deleted_at is null",
        id,
    )
    .execute(&mut *transaction)
    .await?;

    session_db::delete_sessions_by_account(&mut *transaction, id).await?;
    follow_db::delete_follows_by_account(&mut *transaction, id).await?;

    // Leave all conversations: set exited, then delete any now-empty conversations
    sqlx::query!(
        "update conversation_member
         set exited = now()
         where account_id = $1 and exited is null",
        id,
    )
    .execute(&mut *transaction)
    .await?;

    sqlx::query!(
        "with empty_conversations as (
             select conversation.id
             from conversation
             where not exists (
                 select 1 from conversation_member
                 where conversation_member.conversation_id = conversation.id
                   and conversation_member.exited is null
             )
         )
         delete from conversation
         where id in (select id from empty_conversations)",
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;
    Ok(())
}
