use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::game_model::GameEntity;

pub async fn create_game(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    creator_id: Uuid,
    max_players: i32,
) -> Result<GameEntity, AppError> {
    let record = sqlx::query_as::<_, GameEntity>(
        "insert into game (id, name, creator_id, max_players) \
         values ($1, $2, $3, $4) \
         returning *",
    )
    .bind(id)
    .bind(name)
    .bind(creator_id)
    .bind(max_players)
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn get_game_by_id(pool: &PgPool, id: Uuid) -> Result<Option<GameEntity>, AppError> {
    let record = sqlx::query_as::<_, GameEntity>(
        "select g.id, g.name, g.creator_id, g.status, g.max_players, g.created, g.updated \
         from game g \
         where g.id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(record)
}
