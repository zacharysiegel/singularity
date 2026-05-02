use shared::schema::conversation::CreateConversationRequest;
use shared::schema::ws_message::ConnectionType;
use sqlx::PgPool;
use uuid::Uuid;

use super::conversation_broadcast;
use super::conversation_db;
use super::conversation_model::{Conversation, ConversationEntity};
use crate::lobby_error::{LobbyError, OptionExtLobbyError};

pub async fn create_conversation(
    pool: &PgPool,
    command: CreateConversationRequest,
    creator_account_id: Uuid,
    game_id: Option<Uuid>,
) -> Result<Conversation, LobbyError> {
    let mut all_member_ids: Vec<Uuid> = command.member_account_ids.clone();
    if !all_member_ids.contains(&creator_account_id) {
        all_member_ids.push(creator_account_id);
    }

    let duplicate_exists: bool =
        conversation_db::conversation_with_members_exists(pool, game_id, &all_member_ids).await?;
    if duplicate_exists {
        return Err(LobbyError::conflict("a conversation with this member set already exists"));
    }

    let entity: ConversationEntity = conversation_db::create_conversation(
        pool,
        command.name.as_deref(),
        game_id,
        creator_account_id,
        &command.member_account_ids,
    )
    .await?;

    let connection_type: ConnectionType = ConnectionType::from_game_id(game_id);
    for member_id in &all_member_ids {
        conversation_broadcast::broadcast_member_joined(
            pool, entity.id, *member_id, entity.created, connection_type,
        ).await;
    }

    Ok(Conversation::from(entity))
}

pub async fn connection_type_for_conversation(
    pool: &PgPool,
    conversation_id: Uuid,
) -> Result<ConnectionType, LobbyError> {
    let entity: ConversationEntity =
        conversation_db::get_conversation_by_id(pool, conversation_id).await?.or_not_found()?;
    Ok(ConnectionType::from_game_id(entity.game_id))
}
