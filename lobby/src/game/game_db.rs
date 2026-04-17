use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::game_model::{GameBrowserRow, GameEntity};

pub async fn create_game(
    pool: &PgPool,
    id: Uuid,
    name: &str,
    creator_id: Uuid,
    max_players: i32,
) -> Result<GameEntity, AppError> {
    let record = sqlx::query_as!(
        GameEntity,
        "insert into game (id, name, creator_id, max_players) \
         values ($1, $2, $3, $4) \
         returning *",
        id,
        name,
        creator_id,
        max_players,
    )
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn get_game_by_id(pool: &PgPool, id: Uuid) -> Result<Option<GameEntity>, AppError> {
    let record = sqlx::query_as!(
        GameEntity,
        "select g.id, g.name, g.creator_id, g.status, g.max_players, g.created, g.updated \
         from game g \
         where g.id = $1",
        id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn list_games(pool: &PgPool, status: Option<i32>) -> Result<Vec<GameBrowserRow>, AppError> {
    let records = sqlx::query_as!(
        GameBrowserRow,
        "select g.id, g.name, g.creator_id, g.status, g.max_players, \
                coalesce(mcv.member_count, 0) as \"member_count!\", \
                g.created \
         from game g \
         left join member_count_view mcv on mcv.game_id = g.id \
         where ($1::int is null or g.status = $1) \
         order by g.created desc",
        status,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}
