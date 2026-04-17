use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::accolade_model::AccoladeEntity;

pub async fn get_accolades_by_account(pool: &PgPool, account_id: Uuid) -> Result<Vec<AccoladeEntity>, AppError> {
    let records: Vec<AccoladeEntity> = sqlx::query_as!(
        AccoladeEntity,
        "select accolade.id, accolade.account_id, accolade.game_id, accolade.accolade_type, accolade.awarded
         from accolade
         where accolade.account_id = $1
         order by accolade.awarded desc",
        account_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}

pub async fn get_accolades_by_game(pool: &PgPool, game_id: Uuid) -> Result<Vec<AccoladeEntity>, AppError> {
    let records: Vec<AccoladeEntity> = sqlx::query_as!(
        AccoladeEntity,
        "select accolade.id, accolade.account_id, accolade.game_id, accolade.accolade_type, accolade.awarded
         from accolade
         where accolade.game_id = $1
         order by accolade.awarded desc",
        game_id,
    )
    .fetch_all(pool)
    .await?;
    Ok(records)
}
