use shared::error::AppError;
use sqlx::PgPool;
use uuid::Uuid;

use super::game_membership_model::GameMembershipEntity;

/// Atomically checks that the user is not already a member
/// *and* current_players < max_players
/// Then inserts the membership association
pub async fn create_membership_if_available(
    pool: &PgPool,
    game_id: Uuid,
    account_id: Uuid,
) -> Result<Option<GameMembershipEntity>, AppError> {
    let record: Option<GameMembershipEntity> = sqlx::query_as!(
        GameMembershipEntity,
        "insert into game_membership (game_id, account_id)
         select $1, $2
         where not exists (
             select 1 from game_membership where game_id = $1 and account_id = $2
         )
         and (
             select count(*) from game_membership where game_id = $1
         ) < (
             select max_players from game where id = $1
         )
         returning *",
        game_id,
        account_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(record)
}

pub async fn get_membership(
    pool: &PgPool,
    game_id: Uuid,
    account_id: Uuid,
) -> Result<Option<GameMembershipEntity>, AppError> {
    let record: Option<GameMembershipEntity> = sqlx::query_as!(
        GameMembershipEntity,
        "select gm.game_id, gm.account_id, gm.joined
         from game_membership gm
         where gm.game_id = $1 and gm.account_id = $2",
        game_id,
        account_id,
    )
    .fetch_optional(pool)
    .await?;
    Ok(record)
}
