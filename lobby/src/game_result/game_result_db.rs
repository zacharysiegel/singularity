use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::game_result_model::GameResultEntity;

pub async fn get_results_by_game(pool: &PgPool, game_id: Uuid) -> Result<Vec<GameResultEntity>, AppError> {
    let records = sqlx::query_as!(
        GameResultEntity,
        "select gr.game_id, gr.account_id, gr.placement, gr.accolades, gr.stats \
         from game_result gr \
         where gr.game_id = $1 \
         order by gr.placement asc",
        game_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}

pub async fn get_results_by_account(pool: &PgPool, account_id: Uuid) -> Result<Vec<GameResultEntity>, AppError> {
    let records = sqlx::query_as!(
        GameResultEntity,
        "select gr.game_id, gr.account_id, gr.placement, gr.accolades, gr.stats \
         from game_result gr \
         where gr.account_id = $1 \
         order by gr.game_id desc",
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}
