use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::statistic_model::StatisticEntity;

pub async fn get_statistics_by_account(pool: &PgPool, account_id: Uuid) -> Result<Vec<StatisticEntity>, AppError> {
    let records: Vec<StatisticEntity> = sqlx::query_as!(
        StatisticEntity,
        "select statistic.id, statistic.account_id, statistic.game_id,
                statistic.statistic_type, statistic.value, statistic.updated
         from statistic
         where statistic.account_id = $1
         order by statistic.statistic_type asc",
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}

pub async fn get_statistics_by_game(pool: &PgPool, game_id: Uuid) -> Result<Vec<StatisticEntity>, AppError> {
    let records: Vec<StatisticEntity> = sqlx::query_as!(
        StatisticEntity,
        "select statistic.id, statistic.account_id, statistic.game_id,
                statistic.statistic_type, statistic.value, statistic.updated
         from statistic
         where statistic.game_id = $1
         order by statistic.statistic_type asc",
        game_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}
