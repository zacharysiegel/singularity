use std::sync::Arc;

use chrono::{DateTime, Utc};
use shared::schema::conversation::ConversationMemberChangeSerial;
use shared::schema::ws_message::{ConnectionType, WsEvent};
use sqlx::PgPool;
use uuid::Uuid;

use super::conversation_db;
use crate::ws::connection_registry;

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
