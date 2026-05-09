use actix_web::{web, HttpRequest, HttpResponse};
use chrono::Utc;
use shared::schema::conversation::{
    AddConversationMemberRequest, ConversationMemberSerial, ConversationSerial, CreateConversationRequest,
};
use shared::schema::ws_message::ConnectionType;
use sqlx::PgPool;
use uuid::Uuid;
use crate::conversation_message::conversation_message_api;
use crate::game_membership::game_membership_db;
use crate::game_membership::game_membership_model::GameMembershipEntity;
use crate::http;
use crate::lobby_error::{LobbyError, OptionExtLobbyError, ResultExtLobbyError};
use crate::session::session_extractor::AuthenticatedAccount;
use super::conversation;
use super::conversation_broadcast;
use super::conversation_db;
use super::conversation_model::{Conversation, ConversationEntity, ConversationMember, ConversationMemberEntity};

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/conversation")
            .route("", web::post().to(create_conversation))
            .route("", web::get().to(list_conversations))
            .route("/{conversation_id}", web::get().to(get_conversation))
            .route("/{conversation_id}/member", web::post().to(add_member))
            .route("/{conversation_id}/member", web::get().to(get_members))
            .route("/{conversation_id}/leave", web::post().to(leave_conversation))
            .configure(conversation_message_api::conversation_configurer),
    );
}

pub fn game_configurer(config: &mut web::ServiceConfig) {
    config
        .route("/{game_id}/conversation", web::get().to(list_game_conversations))
        .route("/{game_id}/conversation", web::post().to(create_game_conversation));
}

async fn create_conversation(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let payload: CreateConversationRequest = http::deserialize_request(&request, &body).or_bad_request()?;
    if !payload.is_valid() {
        return Err(LobbyError::bad_request("invalid create conversation request"));
    }

    let conversation: Conversation =
        conversation::create_conversation(pool.get_ref(), payload, auth.account_id, None).await?;
    let serial: ConversationSerial = ConversationSerial::from(&conversation);
    Ok(http::serialize_response(&request, &serial))
}

async fn list_conversations(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let conversation_entities: Vec<ConversationEntity> =
        conversation_db::get_conversations_by_account(pool.get_ref(), auth.account_id).await?;

    let conversation_serials: Vec<ConversationSerial> = conversation_entities
        .into_iter()
        .map(|entity| {
            let conversation: Conversation = Conversation::from(entity);
            ConversationSerial::from(&conversation)
        })
        .collect();

    Ok(http::serialize_response(&request, &conversation_serials))
}

async fn get_conversation(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (conversation_id_string,): (String,) = path.into_inner();
    let conversation_id: Uuid = Uuid::parse_str(&conversation_id_string).or_bad_request()?;

    let member: Option<ConversationMemberEntity> =
        conversation_db::get_member(pool.get_ref(), conversation_id, auth.account_id).await?;
    member.or_forbidden("not a member of this conversation")?;

    let entity: Option<ConversationEntity> =
        conversation_db::get_conversation_by_id(pool.get_ref(), conversation_id).await?;
    let entity: ConversationEntity = entity.or_not_found()?;

    let conversation: Conversation = Conversation::from(entity);
    let serial: ConversationSerial = ConversationSerial::from(&conversation);
    Ok(http::serialize_response(&request, &serial))
}

async fn add_member(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (conversation_id_string,): (String,) = path.into_inner();
    let conversation_id: Uuid = Uuid::parse_str(&conversation_id_string).or_bad_request()?;

    let caller_member: Option<ConversationMemberEntity> =
        conversation_db::get_member(pool.get_ref(), conversation_id, auth.account_id).await?;
    caller_member.or_forbidden("not a member of this conversation")?;

    let payload: AddConversationMemberRequest = http::deserialize_request(&request, &body).or_bad_request()?;

    let existing_member: Option<ConversationMemberEntity> =
        conversation_db::get_member(pool.get_ref(), conversation_id, payload.account_id).await?;
    existing_member.then_conflict("account is already an active member")?;

    let member: ConversationMemberEntity = conversation_db::add_member(
        pool.get_ref(),
        conversation_id,
        payload.account_id
    ).await?;

    let connection_type: ConnectionType = conversation::connection_type_for_conversation(pool.get_ref(), conversation_id).await?;
    conversation_broadcast::broadcast_member_joined(
        pool.get_ref(), conversation_id, payload.account_id, member.entered, connection_type,
    ).await;

    let member: ConversationMember = ConversationMember::from(member);
    let member: ConversationMemberSerial = ConversationMemberSerial::from(&member);
    Ok(http::serialize_response(&request, &member))
}

async fn leave_conversation(
    _request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (conversation_id_string,): (String,) = path.into_inner();
    let conversation_id: Uuid = Uuid::parse_str(&conversation_id_string).or_bad_request()?;

    let conversation_entity: ConversationEntity =
        conversation_db::get_conversation_by_id(pool.get_ref(), conversation_id).await?.or_not_found()?;
    if conversation_entity.game_id.is_some() {
        return Err(LobbyError::forbidden("cannot leave in-game conversations"));
    }

    let did_leave: bool = conversation_db::leave_conversation(pool.get_ref(), conversation_id, auth.account_id).await?;

    if did_leave {
        let connection_type: ConnectionType = ConnectionType::from_game_id(conversation_entity.game_id);
        conversation_broadcast::broadcast_member_left(
            pool.get_ref(), conversation_id, auth.account_id, Utc::now(), connection_type,
        ).await;
    }

    match did_leave {
        true => Ok(HttpResponse::Ok().finish()),
        false => Err(LobbyError::not_found("active membership")),
    }
}

async fn get_members(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (conversation_id_string,): (String,) = path.into_inner();
    let conversation_id: Uuid = Uuid::parse_str(&conversation_id_string).or_bad_request()?;

    let caller_member: Option<ConversationMemberEntity> =
        conversation_db::get_member(pool.get_ref(), conversation_id, auth.account_id).await?;
    caller_member.or_forbidden("not a member of this conversation")?;

    let member_entities: Vec<ConversationMemberEntity> =
        conversation_db::get_active_members(pool.get_ref(), conversation_id).await?;

    let member_serials: Vec<ConversationMemberSerial> = member_entities
        .into_iter()
        .map(|entity| {
            let member: ConversationMember = ConversationMember::from(entity);
            ConversationMemberSerial::from(&member)
        })
        .collect();

    Ok(http::serialize_response(&request, &member_serials))
}

async fn list_game_conversations(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (game_id_string,): (String,) = path.into_inner();
    let game_id: Uuid = Uuid::parse_str(&game_id_string).or_bad_request()?;

    let membership: Option<GameMembershipEntity> =
        game_membership_db::get_membership(pool.get_ref(), game_id, auth.account_id).await?;
    membership.or_forbidden("must be a game member to list conversations")?;

    let conversation_entities: Vec<ConversationEntity> =
        conversation_db::get_conversations_by_game_and_account(pool.get_ref(), game_id, auth.account_id).await?;

    let conversation_serials: Vec<ConversationSerial> = conversation_entities
        .into_iter()
        .map(|entity| {
            let conversation: Conversation = Conversation::from(entity);
            ConversationSerial::from(&conversation)
        })
        .collect();

    Ok(http::serialize_response(&request, &conversation_serials))
}

async fn create_game_conversation(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (game_id_string,): (String,) = path.into_inner();
    let game_id: Uuid = Uuid::parse_str(&game_id_string).or_bad_request()?;

    let payload: CreateConversationRequest = http::deserialize_request(&request, &body).or_bad_request()?;
    if !payload.is_valid() {
        return Err(LobbyError::bad_request("invalid create conversation request"));
    }

    let caller_membership: Option<GameMembershipEntity> =
        game_membership_db::get_membership(pool.get_ref(), game_id, auth.account_id).await?;
    caller_membership.or_forbidden("must be a game member to create live conversations")?;

    for member_id in &payload.member_account_ids {
        let membership: Option<GameMembershipEntity> =
            game_membership_db::get_membership(pool.get_ref(), game_id, *member_id).await?;
        if membership.is_none() {
            return Err(LobbyError::bad_request(&format!(
                "account is not a game member; [{member_id}]"
            )));
        }
    }

    let conversation: Conversation =
        conversation::create_conversation(pool.get_ref(), payload, auth.account_id, Some(game_id)).await?;
    let serial: ConversationSerial = ConversationSerial::from(&conversation);
    Ok(http::serialize_response(&request, &serial))
}
