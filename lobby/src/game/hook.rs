use shared::error::AppError;
use shared::schema::game::GameStatus;
use sqlx::PgPool;
use uuid::Uuid;

use crate::conversation::conversation_db;
use crate::game_membership::game_membership_db;

use super::game_model::GameEntity;

pub async fn on_status_transition(
    pool: &PgPool,
    game_entity: &GameEntity,
    new_status: GameStatus,
    auth_account_id: Uuid,
) -> Result<(), AppError> {
    match new_status {
        GameStatus::Active => {
            auto_create_game_conversation(pool, game_entity, auth_account_id).await?;
        }
        GameStatus::Completed => {
            // TODO: finalize scores, award accolades, etc.
        }
        _ => {}
    }
    Ok(())
}

async fn auto_create_game_conversation(
    pool: &PgPool,
    game_entity: &GameEntity,
    creator_account_id: Uuid,
) -> Result<(), AppError> {
    let member_ids: Vec<Uuid> = game_membership_db::get_members_per_game(pool, game_entity.id)
        .await?
        .into_iter()
        .map(|membership| membership.account_id)
        .collect();

    let conversation_name: String = format!("Global [{}]", game_entity.name);

    // creator_account_id has no ownership semantics — it is simply the first member added
    // to the conversation. All members are equal once added.
    conversation_db::create_conversation(
        pool,
        Some(&conversation_name),
        Some(game_entity.id),
        creator_account_id,
        &member_ids,
    )
    .await?;

    Ok(())
}
