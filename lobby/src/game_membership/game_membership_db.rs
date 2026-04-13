use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::game_membership_model::GameMembershipEntity;

pub async fn create_membership(
    pool: &PgPool,
    game_id: Uuid,
    account_id: Uuid,
) -> Result<GameMembershipEntity, AppError> {
    let record = sqlx::query_as::<_, GameMembershipEntity>(
        "insert into game_membership (game_id, account_id) \
         values ($1, $2) \
         returning *",
    )
    .bind(game_id)
    .bind(account_id)
    .fetch_one(pool)
    .await?;
    Ok(record)
}

pub async fn get_membership(
    pool: &PgPool,
    game_id: Uuid,
    account_id: Uuid,
) -> Result<Option<GameMembershipEntity>, AppError> {
    let record = sqlx::query_as::<_, GameMembershipEntity>(
        "select gm.game_id, gm.account_id, gm.joined \
         from game_membership gm \
         where gm.game_id = $1 and gm.account_id = $2",
    )
    .bind(game_id)
    .bind(account_id)
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn count_memberships_by_game(pool: &PgPool, game_id: Uuid) -> Result<i64, AppError> {
    let row: (i64,) = sqlx::query_as(
        "select count(*) from game_membership where game_id = $1",
    )
    .bind(game_id)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}
