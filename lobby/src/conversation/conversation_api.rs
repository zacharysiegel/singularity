use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::conversation::{
    AddConversationMemberRequest, ConversationMemberSerial, ConversationSerial, CreateConversationRequest,
};
use sqlx::PgPool;
use uuid::Uuid;
use crate::conversation_message::conversation_message_api;
use crate::http;
use crate::lobby_error::{LobbyError, OptionExt, ResultExt};
use crate::session::session_extractor::AuthenticatedAccount;
use super::conversation_db;
use super::conversation_model::{Conversation, ConversationEntity, ConversationMember, ConversationMemberEntity};

pub fn configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/conversation")
            .route("", web::post().to(create_conversation))
            .route("", web::get().to(list_conversations))
            .route("/{conversation_id}", web::get().to(get_conversation))
            .route("/{conversation_id}/member", web::post().to(add_member))
            .route("/{conversation_id}/leave", web::post().to(leave_conversation))
            .configure(conversation_message_api::conversation_configurer),
    );
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

    let entity: ConversationEntity = conversation_db::create_conversation(
        pool.get_ref(),
        payload.name.as_deref(),
        None,
        auth.account_id,
        &payload.member_account_ids,
    )
    .await?;
    // todo: should this push to new conversation member websockets? or should we require an http fetch?

    let conversation: Conversation = Conversation::from(entity);
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
    // todo: push to websockets for conversation members

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

    let did_leave: bool = conversation_db::leave_conversation(pool.get_ref(), conversation_id, auth.account_id).await?;
    // todo: push to websockets for conversation members

    match did_leave {
        true => Ok(HttpResponse::Ok().finish()),
        false => Err(LobbyError::not_found("active membership")),
    }
}
