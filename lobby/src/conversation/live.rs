use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::conversation::{ConversationSerial, CreateConversationRequest};
use sqlx::PgPool;
use uuid::Uuid;

use crate::conversation::conversation_db;
use crate::conversation::conversation_model::{Conversation, ConversationEntity};
use crate::game_membership::game_membership_db;
use crate::game_membership::game_membership_model::GameMembershipEntity;
use crate::http;
use crate::lobby_error::{LobbyError, OptionExtLobbyError, ResultExtLobbyError};
use crate::session::session_extractor::AuthenticatedAccount;

pub fn game_configurer(config: &mut web::ServiceConfig) {
    config
        .route("/{game_id}/conversations", web::get().to(list_game_conversations))
        .route("/{game_id}/conversation", web::post().to(create_game_conversation));
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
        conversation_db::get_conversations_by_game(pool.get_ref(), game_id, auth.account_id).await?;

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
    caller_membership.or_forbidden("must be a game member to create conversations")?;

    // Validate all specified members are game members
    for member_id in &payload.member_account_ids {
        let membership: Option<GameMembershipEntity> =
            game_membership_db::get_membership(pool.get_ref(), game_id, *member_id).await?;
        if membership.is_none() {
            return Err(LobbyError::bad_request(&format!(
                "account is not a game member; [{member_id}]"
            )));
        }
    }

    // Include the caller in the full member set
    let mut all_member_ids: Vec<Uuid> = payload.member_account_ids.clone();
    if !all_member_ids.contains(&auth.account_id) {
        all_member_ids.push(auth.account_id);
    }

    // Check for duplicate conversation with the same member set
    let duplicate_exists: bool =
        conversation_db::conversation_with_members_exists(pool.get_ref(), game_id, &all_member_ids).await?;
    if duplicate_exists {
        return Err(LobbyError::conflict("a conversation with this member set already exists for this game"));
    }

    let entity: ConversationEntity = conversation_db::create_conversation(
        pool.get_ref(),
        payload.name.as_deref(),
        Some(game_id),
        auth.account_id,
        &payload.member_account_ids,
    )
    .await?;

    let conversation: Conversation = Conversation::from(entity);
    let serial: ConversationSerial = ConversationSerial::from(&conversation);
    Ok(http::serialize_response(&request, &serial))
}
