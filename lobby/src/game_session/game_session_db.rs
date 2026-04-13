use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::game_session_model::GameSessionEntity;

pub async fn create_game_session(
    pool: &PgPool,
    id: Uuid,
    game_id: Uuid,
    account_id: Uuid,
    session_id: Uuid,
) -> Result<GameSessionEntity, AppError> {
    let record = sqlx::query_as::<_, GameSessionEntity>(
        "insert into game_session (id, game_id, account_id, session_id) \
         values ($1, $2, $3, $4) \
         returning *",
    )
    .bind(id)
    .bind(game_id)
    .bind(account_id)
    .bind(session_id)
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn get_active_game_session(
    pool: &PgPool,
    game_id: Uuid,
    account_id: Uuid,
) -> Result<Option<GameSessionEntity>, AppError> {
    let record = sqlx::query_as::<_, GameSessionEntity>(
        "select gs.id, gs.game_id, gs.account_id, gs.session_id, gs.entered, gs.exited \
         from game_session gs \
         where gs.game_id = $1 and gs.account_id = $2 and gs.exited is null",
    )
    .bind(game_id)
    .bind(account_id)
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn exit_game_session(
    pool: &PgPool,
    game_id: Uuid,
    account_id: Uuid,
) -> Result<Option<GameSessionEntity>, AppError> {
    let record = sqlx::query_as::<_, GameSessionEntity>(
        "update game_session \
         set exited = now() \
         where game_id = $1 and account_id = $2 and exited is null \
         returning *",
    )
    .bind(game_id)
    .bind(account_id)
    .fetch_optional(pool)
    .await?;
    Ok(record)
}
