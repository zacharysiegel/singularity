use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::game_result_model::GameResultEntity;

pub async fn get_results_by_game(pool: &PgPool, game_id: Uuid) -> Result<Vec<GameResultEntity>, AppError> {
    let records: Vec<GameResultEntity> = sqlx::query_as!(
        GameResultEntity,
        "select game_result.game_id, game_result.account_id, game_result.placement
         from game_result
         where game_result.game_id = $1
         order by game_result.placement asc",
        game_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}

pub async fn get_results_by_account(pool: &PgPool, account_id: Uuid) -> Result<Vec<GameResultEntity>, AppError> {
    let records: Vec<GameResultEntity> = sqlx::query_as!(
        GameResultEntity,
        "select game_result.game_id, game_result.account_id, game_result.placement
         from game_result
         where game_result.account_id = $1
         order by game_result.game_id desc",
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}
