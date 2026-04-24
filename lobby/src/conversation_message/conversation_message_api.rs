use actix_web::{web, HttpRequest, HttpResponse};
use shared::schema::conversation_message::{ConversationMessageQuery, ConversationMessageSerial, SendMessageRequest};
use sqlx::PgPool;
use uuid::Uuid;

use crate::conversation::conversation_db;
use crate::conversation::conversation_model::ConversationMemberEntity;
use crate::http;
use crate::lobby_error::{LobbyError, OptionExt, ResultExt};
use crate::session::session_extractor::AuthenticatedAccount;
use super::conversation_message_db;
use super::conversation_message_model::ConversationMessageRow;

const DEFAULT_MESSAGE_LIMIT: i64 = 50;

pub fn conversation_configurer(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/{conversation_id}/messages")
            .route(web::get().to(get_messages))
            .route(web::post().to(send_message)),
    );
}

async fn get_messages(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    query: web::Query<ConversationMessageQuery>,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (conversation_id_string,): (String,) = path.into_inner();
    let conversation_id: Uuid = Uuid::parse_str(&conversation_id_string).or_bad_request()?;

    let member: Option<ConversationMemberEntity> =
        conversation_db::get_member(pool.get_ref(), conversation_id, auth.account_id).await?;
    member.or_forbidden("not a member of this conversation")?;

    let limit: i64 = query.limit.unwrap_or(DEFAULT_MESSAGE_LIMIT);

    let message_rows: Vec<ConversationMessageRow> =
        conversation_message_db::get_messages_by_conversation(pool.get_ref(), conversation_id, limit, query.before)
            .await?;

    let message_serials: Vec<ConversationMessageSerial> = message_rows
        .iter()
        .map(ConversationMessageSerial::from)
        .collect();

    Ok(http::serialize_response(&request, &message_serials))
}

async fn send_message(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<(String,)>,
    body: web::Bytes,
    auth: AuthenticatedAccount,
) -> Result<HttpResponse, LobbyError> {
    let (conversation_id_string,): (String,) = path.into_inner();
    let conversation_id: Uuid = Uuid::parse_str(&conversation_id_string).or_bad_request()?;

    let member: Option<ConversationMemberEntity> =
        conversation_db::get_member(pool.get_ref(), conversation_id, auth.account_id).await?;
    member.or_forbidden("not a member of this conversation")?;

    let payload: SendMessageRequest = http::deserialize_request(&request, &body).or_bad_request()?;

    if !payload.is_valid() {
        return Err(LobbyError::bad_request("invalid send message request"));
    }

    let message_row: ConversationMessageRow =
        conversation_message_db::create_message(pool.get_ref(), conversation_id, auth.account_id, &payload.content)
            .await?;

    let serial: ConversationMessageSerial = ConversationMessageSerial::from(&message_row);
    Ok(http::serialize_response(&request, &serial))
}
