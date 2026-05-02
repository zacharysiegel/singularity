use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse};
use chrono::{DateTime, Utc};
use shared::schema::conversation::{ConversationMemberChangeSerial, ConversationSerial, CreateConversationRequest};
use shared::schema::ws_message::{ConnectionType, WsEvent};
use sqlx::PgPool;
use uuid::Uuid;

use super::conversation_db;
use super::conversation_model::{Conversation, ConversationEntity};
use crate::http;
use crate::lobby_error::LobbyError;
use crate::ws::connection_registry;

pub async fn create_conversation(
    request: &HttpRequest,
    pool: &PgPool,
    payload: CreateConversationRequest,
    creator_account_id: Uuid,
    game_id: Option<Uuid>,
) -> Result<HttpResponse, LobbyError> {
    let mut all_member_ids: Vec<Uuid> = payload.member_account_ids.clone();
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
        payload.name.as_deref(),
        game_id,
        creator_account_id,
        &payload.member_account_ids,
    )
    .await?;

    let connection_type: ConnectionType = ConnectionType::from_game_id(game_id);
    for member_id in &all_member_ids {
        broadcast_member_joined(pool, entity.id, *member_id, entity.created, connection_type).await;
    }

    let conversation: Conversation = Conversation::from(entity);
    let serial: ConversationSerial = ConversationSerial::from(&conversation);
    Ok(http::serialize_response(request, &serial))
}

pub async fn broadcast_member_joined(
    pool: &PgPool,
    conversation_id: Uuid,
    account_id: Uuid,
    timestamp: DateTime<Utc>,
    connection_type: ConnectionType,
) {
    let ws_event: WsEvent = build_event(WsEvent::MemberJoined, conversation_id, account_id, timestamp);
    let member_ids: Vec<Uuid> = match get_member_ids(pool, conversation_id).await {
        Some(ids) => ids,
        None => return,
    };
    connection_registry::send_to_accounts(&member_ids, connection_type, &Arc::new(ws_event));
}

pub async fn broadcast_member_left(
    pool: &PgPool,
    conversation_id: Uuid,
    leaver_account_id: Uuid,
    timestamp: DateTime<Utc>,
    connection_type: ConnectionType,
) {
    let ws_event: WsEvent = build_event(WsEvent::MemberLeft, conversation_id, leaver_account_id, timestamp);
    let mut member_ids: Vec<Uuid> = match get_member_ids(pool, conversation_id).await {
        Some(ids) => ids,
        None => return,
    };
    if !member_ids.contains(&leaver_account_id) {
        member_ids.push(leaver_account_id);
    }
    connection_registry::send_to_accounts(&member_ids, connection_type, &Arc::new(ws_event));
}

fn build_event(
    variant: fn(ConversationMemberChangeSerial) -> WsEvent,
    conversation_id: Uuid,
    account_id: Uuid,
    timestamp: DateTime<Utc>,
) -> WsEvent {
    let serial: ConversationMemberChangeSerial = ConversationMemberChangeSerial {
        conversation_id,
        account_id,
        timestamp,
    };
    variant(serial)
}

async fn get_member_ids(pool: &PgPool, conversation_id: Uuid) -> Option<Vec<Uuid>> {
    match conversation_db::get_active_members(pool, conversation_id).await {
        Ok(members) => {
            let member_ids: Vec<Uuid> = members.iter().map(|member| member.account_id).collect();
            Some(member_ids)
        }
        Err(error) => {
            log::error!("Failed to fetch members for broadcast; [{conversation_id}] [{error}]");
            None
        }
    }
}
